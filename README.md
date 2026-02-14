# KeySwitch

Background daemon that sets your Keychron (Q/V/K Pro) keyboard to the correct **Mac** or **Windows** layout when it (re)connects—e.g. after switching your KVM—so you don’t have to flip the physical switch.

- **macOS**: Sets layout to Mac on connect.
- **Windows**: Sets layout to Windows on connect.

Requires a Keychron keyboard with QMK/VIA firmware (Raw HID). See [docs/README.md](docs/README.md) for build, install, and run-at-login instructions.

## Quick start

```bash
cargo build --release
# Run (foreground):
RUST_LOG=info ./target/release/keyswitch
```

## License

MIT or Apache-2.0, at your option.
