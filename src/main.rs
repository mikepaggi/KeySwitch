//! KeySwitch: background daemon that sets Keychron keyboard layout (Mac/Windows)
//! when the keyboard (re)connects, e.g. after a KVM switch.

mod keychron;
mod via;

#[cfg(target_os = "macos")]
mod detect_macos;

#[cfg(target_os = "windows")]
mod detect_windows;

use hidapi::HidApi;
use log::info;

fn main() {
    let _ = env_logger::try_init();

    info!("KeySwitch starting ({} layout on connect)", {
        #[cfg(target_os = "macos")]
        {
            "Mac"
        }
        #[cfg(target_os = "windows")]
        {
            "Windows"
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            "unknown"
        }
    });

    let mut api = match HidApi::new() {
        Ok(a) => a,
        Err(e) => {
            log::error!("Failed to initialize HID API: {}", e);
            std::process::exit(1);
        }
    };

    #[cfg(target_os = "macos")]
    detect_macos::run(&mut api);

    #[cfg(target_os = "windows")]
    detect_windows::run(&mut api);

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        log::error!("Unsupported OS; KeySwitch supports macOS and Windows only.");
        std::process::exit(1);
    }
}
