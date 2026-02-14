//! macOS device connect detection: poll for Keychron Raw HID devices and apply layout.
//! Uses HID enumeration (polling); run loop keeps the daemon alive.

use hidapi::HidApi;
use log::debug;
use std::collections::HashSet;
use std::time::Duration;

use crate::keychron::{apply_to_connected_keychrons, poll_interval};
use crate::via::Layout;

/// Determines layout for this OS: on macOS we set Mac layout.
fn target_layout() -> Layout {
    Layout::Mac
}

/// Runs the detection loop: periodically enumerate Keychron Raw HID devices
/// and apply the target layout to any newly connected device.
pub fn run(api: &mut HidApi) -> ! {
    let mut applied_paths: HashSet<String> = HashSet::new();
    let layout = target_layout();
    let interval = poll_interval();

    loop {
        apply_to_connected_keychrons(api, layout, &mut applied_paths);
        debug!("Poll: {} Keychron device(s) with layout applied", applied_paths.len());
        std::thread::sleep(interval);
    }
}
