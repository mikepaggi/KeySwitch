//! Minimal VIA Raw HID protocol for QMK keyboards.
//! Report format: 32 bytes; first byte 0x00 (command start), then command id + payload.

pub const RAW_EPSIZE: usize = 32;
const COMMAND_START: u8 = 0x00;

// VIA command IDs (from qmk_firmware quantum/via.h)
const ID_SET_KEYBOARD_VALUE: u8 = 0x03;
const ID_GET_KEYBOARD_VALUE: u8 = 0x02;
const ID_LAYOUT_OPTIONS: u8 = 0x02;

/// Layout option value: 0 = Windows, 1 = Mac (Keychron convention).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    Windows = 0,
    Mac = 1,
}

/// Builds the 32-byte Raw HID report to set layout options (Mac/Windows).
/// Value is 32-bit in protocol; typically only the LSB is used (0 = Windows, 1 = Mac).
pub fn set_layout_options_report(layout: Layout) -> [u8; RAW_EPSIZE] {
    let value = layout as u32;
    let mut report = [0u8; RAW_EPSIZE];
    report[0] = COMMAND_START;
    report[1] = ID_SET_KEYBOARD_VALUE;
    report[2] = ID_LAYOUT_OPTIONS;
    // Big-endian 32-bit value (QMK via.c expects command_data[1..4])
    report[3] = (value >> 24) as u8;
    report[4] = (value >> 16) as u8;
    report[5] = (value >> 8) as u8;
    report[6] = value as u8;
    report
}

/// Builds the 32-byte Raw HID report to get layout options (for discovery/debug).
pub fn get_layout_options_report() -> [u8; RAW_EPSIZE] {
    let mut report = [0u8; RAW_EPSIZE];
    report[0] = COMMAND_START;
    report[1] = ID_GET_KEYBOARD_VALUE;
    report[2] = ID_LAYOUT_OPTIONS;
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_layout_report_mac() {
        let r = set_layout_options_report(Layout::Mac);
        assert_eq!(r[0], COMMAND_START);
        assert_eq!(r[1], ID_SET_KEYBOARD_VALUE);
        assert_eq!(r[2], ID_LAYOUT_OPTIONS);
        assert_eq!(r[6], 1);
    }

    #[test]
    fn test_set_layout_report_windows() {
        let r = set_layout_options_report(Layout::Windows);
        assert_eq!(r[6], 0);
    }
}
