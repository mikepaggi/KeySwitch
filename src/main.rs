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

/// Returns true if the given args request daemon mode (--daemon or -d).
pub(crate) fn want_daemon(args: &[String]) -> bool {
    args.iter().any(|a| a == "--daemon" || a == "-d")
}

/// Parses an optional `--layout mac|windows` override from args.
/// When present, this overrides the compile-time OS default — useful for
/// testing Windows layout on a Mac without a VM.
pub(crate) fn parse_layout_override(args: &[String]) -> Option<via::Layout> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--layout" {
            match iter.next().map(|s| s.as_str()) {
                Some("mac") => return Some(via::Layout::Mac),
                Some("windows") => return Some(via::Layout::Windows),
                _ => {}
            }
        }
    }
    None
}

/// If --daemon or -d was passed, detach and run in background (Unix) or spawn a no-window process (Windows).
fn maybe_daemonize() {
    let args: Vec<String> = std::env::args().collect();
    if !want_daemon(&args) {
        return;
    }

    #[cfg(unix)]
    daemonize_unix();

    #[cfg(windows)]
    daemonize_windows();
}

#[cfg(unix)]
fn daemonize_unix() {
    use std::fs::OpenOptions;
    use std::os::unix::io::IntoRawFd;

    // Double-fork so it not a session leader and won't get a controlling terminal.
    let fork1 = unsafe { nix::unistd::fork() };
    match fork1 {
        Ok(nix::unistd::ForkResult::Parent { .. }) => std::process::exit(0),
        Ok(nix::unistd::ForkResult::Child) => {}
        Err(e) => {
            eprintln!("KeySwitch: fork: {}", e);
            std::process::exit(1);
        }
    }
    let _ = nix::unistd::setsid();
    let fork2 = unsafe { nix::unistd::fork() };
    match fork2 {
        Ok(nix::unistd::ForkResult::Parent { .. }) => std::process::exit(0),
        Ok(nix::unistd::ForkResult::Child) => {}
        Err(e) => {
            eprintln!("KeySwitch: fork: {}", e);
            std::process::exit(1);
        }
    }
    let _ = nix::unistd::chdir(std::path::Path::new("/"));

    // Redirect stdin to /dev/null, stdout/stderr to log files (same paths as LaunchAgent plist).
    let devnull = match OpenOptions::new().read(true).open("/dev/null") {
        Ok(f) => f.into_raw_fd(),
        Err(e) => {
            eprintln!("KeySwitch: open /dev/null: {}", e);
            std::process::exit(1);
        }
    };
    let _ = nix::unistd::dup2(devnull, 0);

    let log_out = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/keyswitch.log")
    {
        Ok(f) => f.into_raw_fd(),
        Err(e) => {
            eprintln!("KeySwitch: open /tmp/keyswitch.log: {}", e);
            std::process::exit(1);
        }
    };
    let _ = nix::unistd::dup2(log_out, 1);

    let log_err = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/keyswitch.err.log")
    {
        Ok(f) => f.into_raw_fd(),
        Err(e) => {
            eprintln!("KeySwitch: open /tmp/keyswitch.err.log: {}", e);
            std::process::exit(1);
        }
    };
    let _ = nix::unistd::dup2(log_err, 2);
}

#[cfg(windows)]
fn daemonize_windows() {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let self_exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("KeySwitch: current_exe: {}", e);
            std::process::exit(1);
        }
    };
    let args: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| a != "--daemon" && a != "-d")
        .collect();
    let mut child = Command::new(&self_exe);
    child.args(&args).creation_flags(CREATE_NO_WINDOW);
    match child.spawn() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("KeySwitch: failed to spawn background process: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    maybe_daemonize();

    let _ = env_logger::try_init();

    let args: Vec<String> = std::env::args().collect();
    let layout_override = parse_layout_override(&args);

    #[cfg(target_os = "macos")]
    {
        let layout = layout_override.unwrap_or(detect_macos::target_layout());
        let layout_name = match layout {
            via::Layout::Mac => "Mac",
            via::Layout::Windows => "Windows (override)",
        };
        info!("KeySwitch starting ({} layout on connect)", layout_name);

        let mut api = match HidApi::new() {
            Ok(a) => a,
            Err(e) => {
                log::error!("Failed to initialize HID API: {}", e);
                std::process::exit(1);
            }
        };

        detect_macos::run(&mut api, layout);
    }

    #[cfg(target_os = "windows")]
    {
        let layout = layout_override.unwrap_or(detect_windows::target_layout());
        let layout_name = match layout {
            via::Layout::Mac => "Mac (override)",
            via::Layout::Windows => "Windows",
        };
        info!("KeySwitch starting ({} layout on connect)", layout_name);

        let mut api = match HidApi::new() {
            Ok(a) => a,
            Err(e) => {
                log::error!("Failed to initialize HID API: {}", e);
                std::process::exit(1);
            }
        };

        detect_windows::run(&mut api, layout);
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        log::error!("Unsupported OS; KeySwitch supports macOS and Windows only.");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_layout_override, want_daemon};
    use crate::via::Layout;

    #[test]
    fn test_want_daemon() {
        assert!(!want_daemon(&[]));
        assert!(!want_daemon(&["keyswitch".into()]));
        assert!(want_daemon(&["keyswitch".into(), "--daemon".into()]));
        assert!(want_daemon(&["keyswitch".into(), "-d".into()]));
        assert!(want_daemon(&["/path/to/keyswitch".into(), "-d".into()]));
        assert!(!want_daemon(&["keyswitch".into(), "--other".into()]));
    }

    #[test]
    fn test_parse_layout_override() {
        assert_eq!(parse_layout_override(&[]), None);
        assert_eq!(parse_layout_override(&["keyswitch".into()]), None);
        assert_eq!(
            parse_layout_override(&["keyswitch".into(), "--layout".into(), "mac".into()]),
            Some(Layout::Mac)
        );
        assert_eq!(
            parse_layout_override(&["keyswitch".into(), "--layout".into(), "windows".into()]),
            Some(Layout::Windows)
        );
        // Unknown value is ignored
        assert_eq!(
            parse_layout_override(&["keyswitch".into(), "--layout".into(), "linux".into()]),
            None
        );
        // Missing value is ignored
        assert_eq!(
            parse_layout_override(&["keyswitch".into(), "--layout".into()]),
            None
        );
        // Works alongside --daemon
        assert_eq!(
            parse_layout_override(&[
                "keyswitch".into(),
                "--daemon".into(),
                "--layout".into(),
                "windows".into()
            ]),
            Some(Layout::Windows)
        );
    }
}
