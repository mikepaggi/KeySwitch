//! Keychron (Q/V/K Pro) device matching and layout application via VIA Raw HID.

use hidapi::HidApi;
use log::{info, warn};
use std::collections::HashSet;
use std::time::Duration;

use crate::via::{set_layout_options_report, Layout};

/// Keychron USB vendor ID (from usb-ids / Keychron).
pub const KEYCHRON_VID: u16 = 0x3434;

/// QMK/VIA Raw HID usage page.
pub const RAW_HID_USAGE_PAGE: u16 = 0xFF60;

/// Unique key for a device (for tracking already-applied devices).
fn device_key(vid: u16, pid: u16, serial: Option<&str>) -> String {
    format!(
        "{:04x}:{:04x}:{}",
        vid,
        pid,
        serial.unwrap_or("")
    )
}

/// Sends the SetKeyboardValue(LayoutOptions) command to the open HID device.
pub fn send_layout_to_device(
    device: &hidapi::HidDevice,
    layout: Layout,
) -> Result<(), String> {
    let report = set_layout_options_report(layout);
    let written = device.write(&report).map_err(|e| e.to_string())?;
    if written != report.len() {
        return Err(format!("wrote {} bytes, expected {}", written, report.len()));
    }
    // Read back response (VIA echoes the buffer).
    let mut buf = [0u8; 32];
    let _ = device.set_blocking_mode(true);
    let _ = device.read_timeout(&mut buf[..], 500);
    Ok(())
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
                    Ok(()) => {
                        info!(
                            "Set Keychron {:04x}:{:04x} to {:?} layout",
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
