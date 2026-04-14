# pixelbox

A command-line tool for controlling the Divoom Ditoo Pro over Bluetooth Low Energy from Linux.

Built on [bluer](https://github.com/bluez/bluer), the official Rust interface to the Linux BlueZ stack.

## Installing on Ubuntu

The quickest way is the install script, which downloads the latest release binary and drops it in `/usr/local/bin`:

```sh
curl -fsSL https://raw.githubusercontent.com/dend/pixelbox/main/install.sh | bash
```

It detects your architecture (x86_64 or aarch64), installs BlueZ if missing, and starts the bluetooth service.

### Manual install

Alternatively, download the tarball for your architecture from the [releases page](https://github.com/dend/pixelbox/releases), then:

```sh
# Install runtime dependencies
sudo apt install bluetooth bluez
sudo systemctl enable --now bluetooth

# Extract and install the binary
tar xzf pixelbox-*.tar.gz
sudo mv pixelbox /usr/local/bin/

# Verify it works
pixelbox --help
```

If your user isn't in the `bluetooth` group, add yourself so you can scan and connect without root:

```sh
sudo usermod -aG bluetooth $USER
```

Log out and back in for the group change to take effect.

## Building from source

You need Rust and a few system headers:

```sh
sudo apt install libdbus-1-dev pkg-config bluetooth bluez
sudo systemctl enable --now bluetooth
```

Then:

```sh
git clone https://github.com/dend/pixelbox.git
cd pixelbox
cargo build --release
```

The binary ends up at `target/release/pixelbox`.

## Pairing

Before pixelbox can talk to your Ditoo Pro, you need to put it into Bluetooth pairing mode:

1. **Hold the power button for 8 seconds.** The LED panel will show a pairing animation when the device is ready.
2. Pair it from your computer using your desktop's Bluetooth settings, or with `bluetoothctl`:

```sh
bluetoothctl
# inside bluetoothctl:
scan on             # wait for "DitooPro" to appear
pair AA:BB:CC:DD:EE:FF
trust AA:BB:CC:DD:EE:FF
connect AA:BB:CC:DD:EE:FF
exit
```

You only need to pair once. After that the device will reconnect automatically when powered on.

## Usage

Every command that talks to the device needs the BLE MAC address, passed with `-a`. You can find the address with the `scan` subcommand.

Commands are grouped: `pixelbox device`, `pixelbox display`, `pixelbox alarm`, `pixelbox audio`, `pixelbox drawing`, `pixelbox text`. Run any group with `--help` to see its subcommands.

### Finding your device

```sh
pixelbox scan
pixelbox scan --duration 15    # scan for 15 seconds instead of the default 5
```

The Ditoo Pro advertises as `DitooPro` over BLE. It's highlighted in the scan output. Grab the address (e.g. `AA:BB:CC:DD:EE:FF`).

### Device settings

```sh
# Sync clock to your system time
pixelbox device set-time -a AA:BB:CC:DD:EE:FF

# Set brightness (0-100)
pixelbox device set-brightness -a AA:BB:CC:DD:EE:FF -l 75

# Rename the device
pixelbox device set-name -a AA:BB:CC:DD:EE:FF -n "My Ditoo"

# Temperature in celsius
pixelbox device set-temp-type -a AA:BB:CC:DD:EE:FF --celsius

# Auto power-off after 30 minutes (0 to disable)
pixelbox device set-auto-power-off -a AA:BB:CC:DD:EE:FF -m 30

# Eye guard on/off
pixelbox device set-eye-guard -a AA:BB:CC:DD:EE:FF --enable

# Set device language (0=en, 3=ja, 9=de, 12=ko, ...)
pixelbox device set-language -a AA:BB:CC:DD:EE:FF -l 0

# Countdown timer
pixelbox device set-countdown -a AA:BB:CC:DD:EE:FF --enable --minutes 5 --seconds 0

# Scoreboard
pixelbox device set-scoreboard -a AA:BB:CC:DD:EE:FF --enable --score1 3 --score2 2

# Game key input
pixelbox device game-key -a AA:BB:CC:DD:EE:FF -k 1
```

### Display modes

```sh
# Switch to clock face
pixelbox display set-mode -a AA:BB:CC:DD:EE:FF -m clock

# Switch to temperature display
pixelbox display set-mode -a AA:BB:CC:DD:EE:FF -m temperature

# Special effect (pass effect ID as --param)
pixelbox display set-mode -a AA:BB:CC:DD:EE:FF -m special --param 3

# Sound-reactive visualizer
pixelbox display set-mode -a AA:BB:CC:DD:EE:FF -m sound --param 0

# Music visualizer
pixelbox display set-mode -a AA:BB:CC:DD:EE:FF -m music --param 1

# Solid color light
pixelbox display set-color-light -a AA:BB:CC:DD:EE:FF --r 255 --g 0 --b 128 --brightness 80

# Preview a color without saving
pixelbox display try-color -a AA:BB:CC:DD:EE:FF --r 0 --g 255 --b 0

# Switch work mode (bluetooth, sd, linein, uac)
pixelbox display change-mode -a AA:BB:CC:DD:EE:FF -m sd

# Screen rotation / mirror / on-off
pixelbox display set-screen-dir -a AA:BB:CC:DD:EE:FF -d 1
pixelbox display set-screen-mirror -a AA:BB:CC:DD:EE:FF --enable
pixelbox display set-screen-on -a AA:BB:CC:DD:EE:FF --on
```

### Alarms and sleep

```sh
# List current alarms
pixelbox alarm get -a AA:BB:CC:DD:EE:FF

# Set alarm 0 for 7:30 AM, all days, volume 50
pixelbox alarm set -a AA:BB:CC:DD:EE:FF --id 0 --enable --hour 7 --minute 30 --weekdays 127 --volume 50

# Sleep mode color and brightness
pixelbox alarm set-sleep-color -a AA:BB:CC:DD:EE:FF --r 20 --g 0 --b 40
pixelbox alarm set-sleep-light -a AA:BB:CC:DD:EE:FF -l 10
```

### Audio

```sh
# Volume
pixelbox audio set-volume -a AA:BB:CC:DD:EE:FF -l 60

# Playback controls
pixelbox audio play-pause -a AA:BB:CC:DD:EE:FF
pixelbox audio next -a AA:BB:CC:DD:EE:FF
pixelbox audio prev -a AA:BB:CC:DD:EE:FF

# Play SD card track by ID
pixelbox audio sd-play -a AA:BB:CC:DD:EE:FF --id 3

# 10-band equalizer (mode + comma-separated band values)
pixelbox audio set-equalizer -a AA:BB:CC:DD:EE:FF -m 1 -b "80,75,70,65,60,60,65,70,75,80"
pixelbox audio reset-equalizer -a AA:BB:CC:DD:EE:FF

# Mic on/off
pixelbox audio set-mic -a AA:BB:CC:DD:EE:FF --on

# Mute the startup sound
pixelbox audio mute-startup -a AA:BB:CC:DD:EE:FF
```

### Drawing and pixel art

The Ditoo Pro has a 16x16 pixel display.

```sh
# Enter drawing pad, draw some pixels, exit
pixelbox drawing pad-enter -a AA:BB:CC:DD:EE:FF
pixelbox drawing pad-draw -a AA:BB:CC:DD:EE:FF -x 0 -y 0 --r 255 --g 0 --b 0
pixelbox drawing pad-draw -a AA:BB:CC:DD:EE:FF -x 8 -y 8 --r 0 --g 255 --b 0
pixelbox drawing pad-exit -a AA:BB:CC:DD:EE:FF

# GIF animation speed
pixelbox drawing set-gif-speed -a AA:BB:CC:DD:EE:FF -s 500

# Enable/disable boot animation
pixelbox drawing set-boot-gif -a AA:BB:CC:DD:EE:FF --enable

# Scroll mode
pixelbox drawing scroll -a AA:BB:CC:DD:EE:FF -m 1 -s 100
```

### Text and notifications

```sh
# Send text to the LED display
pixelbox text send -a AA:BB:CC:DD:EE:FF -t "Hello!"

# Clear the text
pixelbox text clear -a AA:BB:CC:DD:EE:FF

# Scrolling text with speed
pixelbox text scroll-text -a AA:BB:CC:DD:EE:FF -t "Breaking news..." --speed 80

# Set text color / effect / size
pixelbox text set-color -a AA:BB:CC:DD:EE:FF --r 255 --g 200 --b 0
pixelbox text set-effect -a AA:BB:CC:DD:EE:FF -e 2
pixelbox text set-size -a AA:BB:CC:DD:EE:FF -s 1

# Reset notifications
pixelbox text reset-notify -a AA:BB:CC:DD:EE:FF
```

### Querying device info

```sh
pixelbox info -a AA:BB:CC:DD:EE:FF
```

Shows the device name, connection status, and signal strength (no BLE connection needed, reads from BlueZ cache).

## Ditoo Pro specs

- 16x16 pixel LED display
- BLE (Bluetooth Low Energy) connection, not classic SPP
- Speaker, microphone, SD card slot
- No FM radio

## Wire protocol

Packets follow a simple framed format:

```
0x01 | length_lo length_hi | command_id | payload... | checksum_lo checksum_hi | 0x02
```

- Length covers `command_id + payload + checksum` (little-endian u16).
- Checksum is the sum of all bytes from `length_lo` through the end of the payload (little-endian u16, wrapping).
- Bytes 0x01, 0x02, 0x03 inside the frame are escaped: `0x01 -> 0x03 0x04`, `0x02 -> 0x03 0x05`, `0x03 -> 0x03 0x06`.
- BLE writes are chunked to 20 bytes (the Ditoo Pro firmware expects this).

Extended commands are wrapped inside `SPP_DIVOOM_EXTERN_CMD` (0xBD), with the extended command ID as the first payload byte.

## Running tests

```sh
cargo test
```

172 tests covering packet encoding, checksum calculation, escape sequences, and every command builder function.

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
