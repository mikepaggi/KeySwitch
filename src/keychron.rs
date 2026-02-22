//! Keychron (Q/V/K Pro) device matching and layout application via VIA Raw HID.

use hidapi::HidApi;
use log::{info, warn};
use std::collections::HashSet;
use std::time::Duration;

use crate::via::{parse_layout_options_response, set_layout_options_report, Layout};

/// Keychron USB vendor ID (from usb-ids / Keychron).
pub const KEYCHRON_VID: u16 = 0x3434;

/// QMK/VIA Raw HID usage page.
pub const RAW_HID_USAGE_PAGE: u16 = 0xFF60;

/// Unique key for a device (for tracking already-applied devices).
pub(crate) fn device_key(vid: u16, pid: u16, serial: Option<&str>) -> String {
    format!(
        "{:04x}:{:04x}:{}",
        vid,
        pid,
        serial.unwrap_or("")
    )
}

/// Sends the SetKeyboardValue(LayoutOptions) command to the open HID device.
/// VIA echoes the report back verbatim, so we parse the echo as confirmation.
/// Returns the layout the keyboard echoed back, or None if there was no echo.
pub fn send_layout_to_device(
    device: &hidapi::HidDevice,
    layout: Layout,
) -> Result<Option<Layout>, String> {
    let report = set_layout_options_report(layout);
    let written = device.write(&report).map_err(|e| e.to_string())?;
    if written != report.len() {
        return Err(format!("wrote {} bytes, expected {}", written, report.len()));
    }
    // VIA echoes the SET report back verbatim. Byte 6 of the echo contains
    // the layout value we sent, which confirms the keyboard received the command.
    let mut echo = [0u8; 32];
    let _ = device.set_blocking_mode(true);
    match device.read_timeout(&mut echo, 500) {
        Ok(n) if n > 0 => Ok(parse_layout_options_response(&echo)),
        _ => Ok(None),
    }
}

/// Applies the given layout to all currently connected Keychron Raw HID devices
/// that are not yet in `applied_keys`. Newly applied device keys are added to `applied_keys`.
/// Keys no longer present are removed from `applied_keys`.
pub fn apply_to_connected_keychrons(
    api: &mut HidApi,
    layout: Layout,
    applied_keys: &mut HashSet<String>,
) {
    let _ = api.refresh_devices();

    let current_keys: HashSet<String> = api
        .device_list()
        .filter(|d| {
            d.vendor_id() == KEYCHRON_VID && d.usage_page() == RAW_HID_USAGE_PAGE
        })
        .map(|d| device_key(d.vendor_id(), d.product_id(), d.serial_number()))
        .collect();

    applied_keys.retain(|k| current_keys.contains(k));

    for device_info in api
        .device_list()
        .filter(|d| {
            d.vendor_id() == KEYCHRON_VID && d.usage_page() == RAW_HID_USAGE_PAGE
        })
    {
        let key = device_key(
            device_info.vendor_id(),
            device_info.product_id(),
            device_info.serial_number(),
        );
        if applied_keys.contains(&key) {
            continue;
        }
        match device_info.open_device(api) {
            Ok(device) => {
                match send_layout_to_device(&device, layout) {
                    Ok(Some(confirmed)) if confirmed == layout => {
                        info!(
                            "Set Keychron {:04x}:{:04x} to {:?} layout (confirmed by keyboard)",
                            device_info.vendor_id(),
                            device_info.product_id(),
                            layout
                        );
                        applied_keys.insert(key);
                    }
                    Ok(Some(other)) => {
                        warn!(
                            "Layout mismatch on {:04x}:{:04x}: sent {:?}, keyboard reports {:?}",
                            device_info.vendor_id(),
                            device_info.product_id(),
                            layout,
                            other
                        );
                        applied_keys.insert(key);
                    }
                    Ok(None) => {
                        info!(
                            "Set Keychron {:04x}:{:04x} to {:?} layout (no readback from keyboard)",
                            device_info.vendor_id(),
                            device_info.product_id(),
                            layout
                        );
                        applied_keys.insert(key);
                    }
                    Err(e) => {
                        warn!("Failed to set layout on {}: {}", key, e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to open {}: {}", key, e);
            }
        }
    }
}

/// Poll interval for device detection.
pub fn poll_interval() -> Duration {
    Duration::from_secs(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_key_format() {
        assert_eq!(device_key(0x3434, 0x1234, None), "3434:1234:");
        assert_eq!(device_key(0x3434, 0x1234, Some("ABC")), "3434:1234:ABC");
        assert_eq!(device_key(0x0001, 0x0002, Some("")), "0001:0002:");
    }

    #[test]
    fn test_device_key_keychron_vid() {
        assert!(device_key(KEYCHRON_VID, 0x1234, None).starts_with("3434:"));
    }

    #[test]
    fn test_poll_interval() {
        let d = poll_interval();
        assert_eq!(d.as_secs(), 2);
    }

    #[test]
    fn test_send_layout_uses_via_report() {
        let report = crate::via::set_layout_options_report(crate::via::Layout::Mac);
        assert_eq!(report.len(), crate::via::RAW_EPSIZE);
        assert_eq!(report[0], 0x00);
        assert_eq!(report[1], 0x03);
        assert_eq!(report[2], 0x02);
        assert_eq!(report[6], 1);
    }
}
