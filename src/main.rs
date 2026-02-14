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

/// If --daemon or -d was passed, detach and run in background (Unix) or spawn a no-window process (Windows).
fn maybe_daemonize() {
    let args: Vec<String> = std::env::args().collect();
    let want_daemon = args.iter().any(|a| a == "--daemon" || a == "-d");
    if !want_daemon {
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
