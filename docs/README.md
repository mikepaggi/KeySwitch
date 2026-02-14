# KeySwitch – Setup and Reference

## Requirements

- **Rust**: 1.63 or newer (e.g. `rustup update stable`).
- **Keychron**: Q, V, or K Pro series (QMK/VIA firmware). The keyboard must be connected via USB and expose the Raw HID interface (usage page `0xFF60`).

## Building

```bash
cargo build --release
```

Binaries:

- **macOS**: `target/release/keyswitch`
- **Windows**: `target/release/keyswitch.exe`

## Running

- **Foreground (default)**: Runs in the foreground; when a Keychron Raw HID device is detected (or reconnects after a KVM switch), it sets the keyboard layout to **Mac**. Set `RUST_LOG=info` to see log lines (e.g. “Set Keychron … to Mac layout”).
- **Windows**: Same behavior; layout is set to **Windows**.
- **Background**: Run with `--daemon` or `-d` to detach: on macOS the process double-forks and logs to `/tmp/keyswitch.log` and `/tmp/keyswitch.err.log`; on Windows it spawns a no-console process and the original exits.

The daemon polls every 2 seconds for connected Keychron Raw HID devices and applies the layout only to devices that have not been configured in the current run (so it does not spam the keyboard).

## Run at login

### macOS (LaunchAgent)

1. Copy the binary to a path in your `PATH` (e.g. `/usr/local/bin/keyswitch`):

   ```bash
   sudo cp target/release/keyswitch /usr/local/bin/keyswitch
   ```

2. Install the LaunchAgent plist (adjust paths if your binary is elsewhere):

   - Copy `com.keyswitch.daemon.plist` from the project into `~/Library/LaunchAgents/`.
   - Edit the plist and set the `ProgramArguments` path to the real path of the `keyswitch` binary (e.g. `/usr/local/bin/keyswitch` or `/Users/you/Developer/KeySwitch/target/release/keyswitch`).

3. Load and start the agent:

   ```bash
   launchctl load ~/Library/LaunchAgents/com.keyswitch.daemon.plist
   ```

4. To stop:

   ```bash
   launchctl unload ~/Library/LaunchAgents/com.keyswitch.daemon.plist
   ```

Logs (if you kept the plist’s `StandardOutPath` / `StandardErrorPath`): `/tmp/keyswitch.log` and `/tmp/keyswitch.err.log`.

### Windows (run at logon)

- Create a shortcut to `keyswitch.exe` in the **Startup** folder (`shell:startup`), or use Task Scheduler to run `keyswitch.exe` at user logon. To avoid a console window, either run `keyswitch.exe --daemon` (e.g. from a shortcut that runs that command) or use Task Scheduler / a shortcut that runs the exe in the background.

## Finding VID/PID

- **macOS**: System Report → USB (or Bluetooth) → select the keyboard → Vendor ID and Product ID.
- **Windows**: Device Manager → Keychron device → Properties → Details → Hardware Ids; VID and PID appear in the string (e.g. `VID_3434&PID_xxxx`).

KeySwitch matches any Keychron device with Vendor ID `0x3434` and Raw HID usage page `0xFF60`; it does not filter by Product ID so all supported Keychron Q/V/K Pro models are included.

## LayoutOptions (Mac vs Windows)

KeySwitch uses the VIA protocol command **SetKeyboardValue** with value id **LayoutOptions** (0x02). The value is a 32-bit number; typically only the LSB is used:

- **0** = Windows layout  
- **1** = Mac layout  

This is consistent with QMK’s `via_set_layout_options` / `via_get_layout_options` and Keychron’s use of layout options for the Mac/Windows switch. The keyboard stores the value in EEPROM, so it persists across reconnects until KeySwitch (or another tool) changes it again.

## Troubleshooting

- **No effect when keyboard connects**: Ensure the keyboard is QMK/VIA (Q, V, or K Pro). Check that only one process is using the Raw HID device (e.g. close the VIA app while KeySwitch is running).
- **Permission errors (macOS)**: Running from a normal user is usually enough; if you see HID open errors, check System Preferences → Privacy & Security for any HID or USB restrictions.
- **Build fails (Rust too old)**: Run `rustup update stable` and ensure `rustc --version` is 1.63 or newer.
