# pixelbox

A command-line tool for controlling the Divoom Ditoo Pro over Bluetooth Low Energy from Linux.

Built on [bluer](https://github.com/bluez/bluer), the official Rust interface to the Linux BlueZ stack.

## Requirements

- Linux with BlueZ (the standard Bluetooth stack on most distros)
- A Bluetooth adapter that supports BLE
- D-Bus development headers (for building)

On Ubuntu/Debian:

```sh
sudo apt install libdbus-1-dev pkg-config bluetooth bluez
sudo systemctl start bluetooth
```

## Building

```sh
git clone https://github.com/dend/pixelbox.git
cd pixelbox
cargo build --release
```

The binary ends up at `target/release/pixelbox`. Copy it wherever you want or run it with `cargo run --release --`.

## Usage

Every command that talks to the device needs the BLE MAC address, passed with `-a`. You can find the address with the `scan` subcommand.

### Finding your device

```sh
pixelbox scan
pixelbox scan --duration 15    # scan for 15 seconds instead of the default 5
```

The Ditoo Pro advertises as `DitooPro` over BLE. It's highlighted in the scan output. Grab the address (e.g. `AA:BB:CC:DD:EE:FF`).

### Setting the clock

Syncs the device clock to your system time:

```sh
pixelbox set-time -a AA:BB:CC:DD:EE:FF
```

### Setting brightness

```sh
pixelbox set-brightness -a AA:BB:CC:DD:EE:FF -l 75   # 0-100
```

### Querying device info

```sh
pixelbox info -a AA:BB:CC:DD:EE:FF
```

Shows the device name, connection status, and signal strength.

## Protocol library

The CLI only wires up a few commands right now, but the underlying protocol library (`src/commands/`) has full coverage of the Ditoo Pro's capabilities. This is the same binary protocol the official Divoom Android app uses over BLE. You can use it as a Rust library to build your own tooling.

### Ditoo Pro specs

- 16x16 pixel LED display
- BLE (Bluetooth Low Energy) connection, not classic SPP
- Speaker, microphone, SD card slot
- No FM radio

### Command modules

| Module | What it covers |
|---|---|
| `commands::device` | System time, brightness, device name, power on/off, auto power-off, eye guard, temperature units, game input, countdown/stopwatch/scoreboard/noise tools, language, connection config |
| `commands::display` | Display modes (clock, temperature, color light, special effects, sound reactive, music visualization), screen rotation/mirror/on-off, light effects, work mode switching (BT/SD/LineIn/USB) |
| `commands::alarm` | Alarms with scene/GIF, alarm preview, alarm volume, voice alarms (record/play/stop), sleep timer with color/light/scene, auto power-off |
| `commands::audio` | Volume, play/pause/skip, SD card music (select/seek/list), equalizer (10-band), mixer, sound control, microphone, karaoke, voice recording, power-on sound config |
| `commands::drawing` | 16x16 pixel art (static and animated), real-time drawing pad, movie playback, user GIF management, sand paint, scrolling, boot animation, GIF speed/timing |
| `commands::text` | LED text (UTF-16LE), scrolling text, 32-pixel text attributes (color/effect/size/speed/frame), notifications with color, daily time messages |

### Wire protocol

Packets follow a simple framed format:

```
0x01 | length_lo length_hi | command_id | payload... | checksum_lo checksum_hi | 0x02
```

- Length covers `command_id + payload + checksum` (little-endian u16).
- Checksum is the sum of all bytes from `length_lo` through the end of the payload (little-endian u16, wrapping).
- Bytes 0x01, 0x02, 0x03 inside the frame are escaped: `0x01 -> 0x03 0x04`, `0x02 -> 0x03 0x05`, `0x03 -> 0x03 0x06`.
- BLE writes are chunked to 20 bytes (the Ditoo Pro firmware expects this).

Extended commands are wrapped inside `SPP_DIVOOM_EXTERN_CMD` (0xBD), with the extended command ID as the first payload byte.

### Example: using the library directly

```rust
use pixelbox::commands::{device, display, drawing};

// Set brightness to 60%
let pkt = device::set_brightness(60);
let bytes = pkt.encode();
// -> write `bytes` to BLE characteristic in 20-byte chunks

// Switch to clock display
let pkt = display::set_box_mode_clock();

// Set a single pixel on the drawing pad (red at position 8,4)
let enter = drawing::drawing_pad_enter(0, 0, 0);
let draw = drawing::drawing_pad_ctrl(8, 4, 255, 0, 0);
let exit = drawing::drawing_pad_exit();
```

## Running tests

```sh
cargo test
```

There are 176 tests covering packet encoding, checksum calculation, escape sequences, and every command builder function.

## BLE characteristic

All communication happens on a single GATT characteristic:

```
Service: discovered dynamically
Characteristic: 49535343-1e4d-4bd9-ba61-23c647249616
Descriptor (CCCD): 00002902-0000-1000-8000-00805f9b34fb
Write type: WRITE_NO_RESPONSE, 20-byte chunks
```

## License

MIT. See [LICENSE](LICENSE).
