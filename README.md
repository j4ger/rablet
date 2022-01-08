# tabletd
A tablet driver written in Rust.

## Roadmap

### Basic functionality

- Select USB device
- Parse input (use the device I have at hand)
- Send input as emulated hid-device/directly using Windows apis
- Windows Radial support (after all that's what this project initially targeted to accomplish)
- Serial debugging tool

### Gui

- Unroll-rs, which I'm currently working on

### Plugin/Module system

- json configurations for generalized devices
- api for invoking event handlers
- external parser support
- plugins for advanced usage

### More

- Bluetooth devices?
- Cross-platform support?
