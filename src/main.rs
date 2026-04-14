#[allow(dead_code)]
mod commands;
#[allow(dead_code)]
mod protocol;

use anyhow::{Context, Result};
use bluer::{Adapter, Address, Device, gatt::remote::Characteristic};
use clap::{Parser, Subcommand};
use protocol::Packet;

const DIVOOM_CHARACTERISTIC: bluer::Uuid =
    bluer::Uuid::from_u128(0x49535343_1e4d_4bd9_ba61_23c647249616);

#[derive(Parser)]
#[command(name = "pixelbox", about = "Control Divoom Ditoo Pro over BLE")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Scan for nearby Divoom devices
    Scan {
        #[arg(short, long, default_value = "5")]
        duration: u64,
    },
    /// Query device info from BlueZ (no connection needed)
    Info {
        #[arg(short, long)]
        address: String,
    },
    /// Device settings: time, brightness, name, power, temperature, language
    #[command(subcommand)]
    Device(DeviceCmd),
    /// Display modes, screen control, light effects
    #[command(subcommand)]
    Display(DisplayCmd),
    /// Alarms, sleep, timers
    #[command(subcommand)]
    Alarm(AlarmCmd),
    /// Volume, playback, SD music, equalizer, mic
    #[command(subcommand)]
    Audio(AudioCmd),
    /// Pixel art, animations, drawing pad, GIFs
    #[command(subcommand)]
    Drawing(DrawingCmd),
    /// LED text, notifications, scrolling text
    #[command(subcommand)]
    Text(TextCmd),
}

// ---------------------------------------------------------------------------
// Device
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
enum DeviceCmd {
    /// Sync clock to system time
    SetTime {
        #[arg(short, long)]
        address: String,
    },
    /// Get device time
    GetTime {
        #[arg(short, long)]
        address: String,
    },
    /// Set brightness (0-100)
    SetBrightness {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        level: u8,
    },
    /// Get current brightness
    GetBrightness {
        #[arg(short, long)]
        address: String,
    },
    /// Set the device Bluetooth name
    SetName {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        name: String,
    },
    /// Get device info (extended command)
    GetDeviceInfo {
        #[arg(short, long)]
        address: String,
    },
    /// Set power on/off config
    SetPower {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        mode: u8,
        #[arg(long)]
        enable: bool,
        #[arg(long, default_value = "0")]
        r: u8,
        #[arg(long, default_value = "0")]
        g: u8,
        #[arg(long, default_value = "0")]
        b: u8,
    },
    /// Get power on/off config
    GetPower {
        #[arg(short, long)]
        address: String,
    },
    /// Set auto power-off timer (minutes, 0 to disable)
    SetAutoPowerOff {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        minutes: u16,
    },
    /// Get auto power-off setting
    GetAutoPowerOff {
        #[arg(short, long)]
        address: String,
    },
    /// Set power channel
    SetPowerChannel {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        channel: u8,
    },
    /// Get power channel
    GetPowerChannel {
        #[arg(short, long)]
        address: String,
    },
    /// Enable/disable energy saving
    SetEnergy {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Get energy saving status
    GetEnergy {
        #[arg(short, long)]
        address: String,
    },
    /// Enable/disable eye guard
    SetEyeGuard {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Get eye guard status
    GetEyeGuard {
        #[arg(short, long)]
        address: String,
    },
    /// Set temperature display (celsius or fahrenheit)
    SetTempType {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        celsius: bool,
    },
    /// Get temperature type
    GetTempType {
        #[arg(short, long)]
        address: String,
    },
    /// Get device temperature
    GetTemp {
        #[arg(short, long)]
        address: String,
    },
    /// Send game key press
    GameKey {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        key: u8,
    },
    /// Send game key release
    GameKeyUp {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        key: u8,
    },
    /// Enable/disable game
    SetGame {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
        #[arg(long, default_value = "0")]
        param: u8,
    },
    /// Get tool info (countdown/stopwatch/scoreboard/noise)
    GetToolInfo {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        tool_type: u8,
    },
    /// Set countdown timer
    SetCountdown {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
        #[arg(long)]
        minutes: u16,
        #[arg(long)]
        seconds: u16,
    },
    /// Set noise meter
    SetNoise {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        param: u8,
    },
    /// Set scoreboard
    SetScoreboard {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
        #[arg(long)]
        score1: u16,
        #[arg(long)]
        score2: u16,
    },
    /// Set device language (0=en 1=zh-hans 2=zh-hant 3=ja 4=th 5=fr 6=it 8=es 9=de 10=ru 11=pt 12=ko)
    SetLanguage {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        lang: u8,
    },
    /// Enable/disable auto-connect
    SetAutoConnect {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Get auto-connect status
    GetAutoConnect {
        #[arg(short, long)]
        address: String,
    },
    /// Enable/disable volume saving
    SetSaveVolume {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Enable/disable peripheral
    SetPeripheral {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Set talk mode
    SetTalk {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
        #[arg(long, default_value = "0")]
        param: u8,
    },
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
enum DisplayCmd {
    /// Set display mode (clock/temperature/color-light/special/sound/music)
    SetMode {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: String,
        #[arg(long, default_value = "0")]
        param: u8,
    },
    /// Get current display mode
    GetMode {
        #[arg(short, long)]
        address: String,
    },
    /// Set color light mode with RGB
    SetColorLight {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        r: u8,
        #[arg(long)]
        g: u8,
        #[arg(long)]
        b: u8,
        #[arg(long, default_value = "100")]
        brightness: u8,
        #[arg(long)]
        speed: bool,
    },
    /// Get current scene
    GetScene {
        #[arg(short, long)]
        address: String,
    },
    /// Preview a color on the display
    TryColor {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        r: u8,
        #[arg(long)]
        g: u8,
        #[arg(long)]
        b: u8,
    },
    /// Change work mode (bluetooth/linein/sd/uac)
    ChangeMode {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: String,
    },
    /// Set screen rotation direction
    SetScreenDir {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        direction: u8,
    },
    /// Get screen rotation direction
    GetScreenDir {
        #[arg(short, long)]
        address: String,
    },
    /// Enable/disable screen mirroring
    SetScreenMirror {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Turn screen on/off
    SetScreenOn {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        on: bool,
    },
    /// Set light arrow indicator mode
    SetLightArrow {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: u8,
    },
    /// Set custom light effect (7 param bytes, comma-separated)
    SetLightEffect {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        params: String,
    },
    /// Reset light effect to default
    ResetLightEffect {
        #[arg(short, long)]
        address: String,
    },
}

// ---------------------------------------------------------------------------
// Alarm
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
enum AlarmCmd {
    /// Get all alarm settings
    Get {
        #[arg(short, long)]
        address: String,
    },
    /// Set an alarm
    Set {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        id: u8,
        #[arg(long)]
        enable: bool,
        #[arg(long)]
        hour: u8,
        #[arg(long)]
        minute: u8,
        #[arg(long, default_value = "0")]
        mode: u8,
        #[arg(long, default_value = "127")]
        weekdays: u8,
        #[arg(long, default_value = "0")]
        frequency: u8,
        #[arg(long, default_value = "0")]
        speed: u16,
        #[arg(long, default_value = "50")]
        volume: u8,
    },
    /// Set alarm listen (preview)
    SetListen {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
        #[arg(long, default_value = "0")]
        param1: u8,
        #[arg(long, default_value = "0")]
        param2: u8,
    },
    /// Set alarm listen volume
    SetListenVolume {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        volume: u8,
    },
    /// Control alarm voice (0=record, 1=play, 2=stop)
    VoiceCtrl {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        control: u8,
        #[arg(long, default_value = "50")]
        volume: u8,
    },
    /// Set sleep timer
    SetSleepTime {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        config: String,
    },
    /// Get sleep mode
    GetSleepMode {
        #[arg(short, long)]
        address: String,
    },
    /// Set sleep light color
    SetSleepColor {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        r: u8,
        #[arg(long)]
        g: u8,
        #[arg(long)]
        b: u8,
    },
    /// Set sleep light level
    SetSleepLight {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        level: u8,
    },
    /// Set auto power-off timer (minutes, 0 to disable)
    SetAutoPowerOff {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        minutes: u16,
    },
    /// Get auto power-off setting
    GetAutoPowerOff {
        #[arg(short, long)]
        address: String,
    },
}

// ---------------------------------------------------------------------------
// Audio
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
enum AudioCmd {
    /// Set volume (0-100)
    SetVolume {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        level: u8,
    },
    /// Get current volume
    GetVolume {
        #[arg(short, long)]
        address: String,
    },
    /// Toggle play/pause
    PlayPause {
        #[arg(short, long)]
        address: String,
    },
    /// Skip to next track
    Next {
        #[arg(short, long)]
        address: String,
    },
    /// Skip to previous track
    Prev {
        #[arg(short, long)]
        address: String,
    },
    /// Play SD card track by ID
    SdPlay {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        id: u16,
    },
    /// Seek SD card track position
    SdSeek {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        pos: u16,
    },
    /// Set SD card play mode
    SdPlayMode {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: u8,
    },
    /// Get SD card music list
    SdList {
        #[arg(short, long)]
        address: String,
        #[arg(long, default_value = "0")]
        start: u16,
        #[arg(long, default_value = "65535")]
        end: u16,
    },
    /// Get SD card music info
    SdInfo {
        #[arg(short, long)]
        address: String,
    },
    /// Enable/disable sound control
    SetSoundCtrl {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Get sound control status
    GetSoundCtrl {
        #[arg(short, long)]
        address: String,
    },
    /// Set 10-band equalizer (mode + 10 comma-separated band values)
    SetEqualizer {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: u8,
        #[arg(short, long)]
        bands: String,
    },
    /// Reset equalizer
    ResetEqualizer {
        #[arg(short, long)]
        address: String,
    },
    /// Control voice (0=record, 1=play, 2=stop)
    Voice {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        control: u8,
    },
    /// Get voice playback status
    VoiceStatus {
        #[arg(short, long)]
        address: String,
    },
    /// Set mixer mode
    SetMixer {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: u8,
        #[arg(short, long)]
        param: u8,
    },
    /// Toggle microphone
    SetMic {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        on: bool,
    },
    /// Set recording control
    Record {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        control: u8,
    },
    /// Set power-on voice volume
    SetStartupVolume {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        volume: u8,
    },
    /// Mute power-on voice
    MuteStartup {
        #[arg(short, long)]
        address: String,
    },
    /// Enable/disable song name display
    SetSongDisplay {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Get song display status
    GetSongDisplay {
        #[arg(short, long)]
        address: String,
    },
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
enum DrawingCmd {
    /// Send an encoded picture
    SendPic {
        #[arg(short, long)]
        address: String,
        #[arg(long, default_value = "0")]
        pic_id: u8,
        #[arg(short, long)]
        data: String,
    },
    /// Enter drawing pad mode
    PadEnter {
        #[arg(short, long)]
        address: String,
        #[arg(long, default_value = "0")]
        r: u8,
        #[arg(long, default_value = "0")]
        g: u8,
        #[arg(long, default_value = "0")]
        b: u8,
    },
    /// Draw a pixel on the pad
    PadDraw {
        #[arg(short, long)]
        address: String,
        #[arg(short = 'x')]
        x: u8,
        #[arg(short = 'y')]
        y: u8,
        #[arg(long)]
        r: u8,
        #[arg(long)]
        g: u8,
        #[arg(long)]
        b: u8,
    },
    /// Exit drawing pad mode
    PadExit {
        #[arg(short, long)]
        address: String,
    },
    /// Start/stop movie playback
    MoviePlay {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        play: bool,
    },
    /// Start user GIF
    GifStart {
        #[arg(short, long)]
        address: String,
        #[arg(long, default_value = "0")]
        anim_id: u8,
    },
    /// End user GIF
    GifEnd {
        #[arg(short, long)]
        address: String,
    },
    /// Stop sending GIF
    GifStop {
        #[arg(short, long)]
        address: String,
    },
    /// Set GIF animation speed
    SetGifSpeed {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        speed: u16,
    },
    /// Set GIF play time
    SetGifPlayTime {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        time: u16,
    },
    /// Reset GIF play time
    ResetGifPlayTime {
        #[arg(short, long)]
        address: String,
    },
    /// Start sand paint mode
    SandPaintStart {
        #[arg(short, long)]
        address: String,
    },
    /// Set scroll mode and speed
    Scroll {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        mode: u8,
        #[arg(short, long)]
        speed: u16,
    },
    /// Enable/disable boot animation
    SetBootGif {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        enable: bool,
    },
    /// Send GIF start signal
    SendGifStart {
        #[arg(short, long)]
        address: String,
    },
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
enum TextCmd {
    /// Send LED text
    Send {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        text: String,
    },
    /// Clear LED text
    Clear {
        #[arg(short, long)]
        address: String,
    },
    /// Set text color
    SetColor {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        r: u8,
        #[arg(long)]
        g: u8,
        #[arg(long)]
        b: u8,
    },
    /// Set text effect
    SetEffect {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        effect: u8,
    },
    /// Set text size
    SetSize {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        size: u8,
    },
    /// Set text scroll speed
    SetSpeed {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        speed: u16,
    },
    /// Set notification picture color
    SetNotifyPic {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        id: u8,
        #[arg(long)]
        r: u8,
        #[arg(long)]
        g: u8,
        #[arg(long)]
        b: u8,
    },
    /// Send notification text
    SetNotifyText {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        notify_type: u8,
        #[arg(short, long)]
        text: String,
    },
    /// Reset all notifications
    ResetNotify {
        #[arg(short, long)]
        address: String,
    },
    /// Delete leave message
    DeleteLeaveMsg {
        #[arg(short, long)]
        address: String,
    },
    /// Send scrolling text with speed
    ScrollText {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        text: String,
        #[arg(long, default_value = "100")]
        speed: u16,
    },
    /// Get daily time message
    GetDailyTime {
        #[arg(short, long)]
        address: String,
    },
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    match cli.command {
        Cmd::Scan { duration } => cmd_scan(&adapter, duration).await?,
        Cmd::Info { address } => {
            let device = adapter.device(parse_address(&address)?)?;
            cmd_info(&device).await?;
        }
        Cmd::Device(c) => handle_device(&adapter, c).await?,
        Cmd::Display(c) => handle_display(&adapter, c).await?,
        Cmd::Alarm(c) => handle_alarm(&adapter, c).await?,
        Cmd::Audio(c) => handle_audio(&adapter, c).await?,
        Cmd::Drawing(c) => handle_drawing(&adapter, c).await?,
        Cmd::Text(c) => handle_text(&adapter, c).await?,
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_address(s: &str) -> Result<Address> {
    s.parse::<Address>()
        .with_context(|| format!("invalid BLE address: {s}"))
}

async fn connect(adapter: &Adapter, address: &str) -> Result<Characteristic> {
    let addr = parse_address(address)?;
    let device = adapter.device(addr)?;

    if !device.is_connected().await? {
        println!("Connecting to {}...", addr);
        device.connect().await.context("failed to connect")?;
        println!("Connected.");
    }

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    for service in device.services().await? {
        for ch in service.characteristics().await? {
            if ch.uuid().await? == DIVOOM_CHARACTERISTIC {
                return Ok(ch);
            }
        }
    }

    anyhow::bail!("Divoom characteristic not found on {}", addr);
}

async fn send(ch: &Characteristic, packet: &Packet) -> Result<()> {
    let bytes = packet.encode();
    for chunk in bytes.chunks(20) {
        ch.write(chunk).await.context("BLE write failed")?;
    }
    Ok(())
}

async fn send_ok(adapter: &Adapter, address: &str, packet: Packet, msg: &str) -> Result<()> {
    let ch = connect(adapter, address).await?;
    send(&ch, &packet).await?;
    println!("{}", msg);
    Ok(())
}

async fn send_query(adapter: &Adapter, address: &str, packet: Packet) -> Result<()> {
    let ch = connect(adapter, address).await?;
    send(&ch, &packet).await?;
    println!("Query sent. Response will arrive via BLE notification.");
    Ok(())
}

fn parse_hex_or_bytes(s: &str) -> Result<Vec<u8>> {
    s.split(',')
        .map(|v| {
            let v = v.trim();
            if let Some(hex) = v.strip_prefix("0x") {
                u8::from_str_radix(hex, 16).with_context(|| format!("bad hex: {v}"))
            } else {
                v.parse::<u8>().with_context(|| format!("bad byte: {v}"))
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Scan & Info
// ---------------------------------------------------------------------------

async fn cmd_scan(adapter: &Adapter, duration_secs: u64) -> Result<()> {
    use bluer::AdapterEvent;
    use tokio_stream::StreamExt;

    println!("Scanning for {} seconds...", duration_secs);
    let discover = adapter.discover_devices().await?;
    tokio::pin!(discover);
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(duration_secs));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(event) = discover.next() => {
                if let AdapterEvent::DeviceAdded(addr) = event
                    && let Ok(device) = adapter.device(addr)
                {
                    let name = device.name().await.ok().flatten();
                    if let Some(ref n) = name {
                        let lower = n.to_lowercase();
                        if lower.contains("ditoo") || lower.contains("divoom")
                            || lower.contains("timebox") || lower.contains("tivoo")
                            || lower.contains("pixoo")
                        {
                            println!("  [DIVOOM] {} - {}", addr, n);
                        } else {
                            println!("          {} - {}", addr, n);
                        }
                    }
                }
            }
            () = &mut timeout => {
                println!("Scan complete.");
                break;
            }
        }
    }
    Ok(())
}

async fn cmd_info(device: &Device) -> Result<()> {
    let name = device.name().await.ok().flatten().unwrap_or_default();
    println!("Device: {}", name);
    println!("Address: {}", device.address());
    println!(
        "Connected: {}",
        device.is_connected().await.unwrap_or(false)
    );
    if let Some(r) = device.rssi().await.ok().flatten() {
        println!("RSSI: {} dBm", r);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Device handler
// ---------------------------------------------------------------------------

async fn handle_device(adapter: &Adapter, cmd: DeviceCmd) -> Result<()> {
    use commands::device;
    match cmd {
        DeviceCmd::SetTime { address } => {
            send_ok(adapter, &address, device::set_time_now(), "Time synced.").await
        }
        DeviceCmd::GetTime { address } => {
            send_query(adapter, &address, device::get_system_time()).await
        }
        DeviceCmd::SetBrightness { address, level } => {
            send_ok(
                adapter,
                &address,
                device::set_brightness(level.min(100)),
                &format!("Brightness set to {}%.", level.min(100)),
            )
            .await
        }
        DeviceCmd::GetBrightness { address } => {
            send_query(adapter, &address, device::get_brightness()).await
        }
        DeviceCmd::SetName { address, name } => {
            send_ok(
                adapter,
                &address,
                device::set_device_name(&name),
                &format!("Name set to '{}'.", name),
            )
            .await
        }
        DeviceCmd::GetDeviceInfo { address } => {
            send_query(adapter, &address, device::get_device_info()).await
        }
        DeviceCmd::SetPower {
            address,
            mode,
            enable,
            r,
            g,
            b,
        } => {
            send_ok(
                adapter,
                &address,
                device::set_power_on_off(mode, enable, r, g, b),
                "Power config set.",
            )
            .await
        }
        DeviceCmd::GetPower { address } => {
            send_query(adapter, &address, device::get_power_on_off()).await
        }
        DeviceCmd::SetAutoPowerOff { address, minutes } => {
            send_ok(
                adapter,
                &address,
                commands::alarm::set_auto_power_off(minutes),
                &format!("Auto power-off set to {} min.", minutes),
            )
            .await
        }
        DeviceCmd::GetAutoPowerOff { address } => {
            send_query(adapter, &address, commands::alarm::get_auto_power_off()).await
        }
        DeviceCmd::SetPowerChannel { address, channel } => {
            send_ok(
                adapter,
                &address,
                device::set_power_channel(channel),
                "Power channel set.",
            )
            .await
        }
        DeviceCmd::GetPowerChannel { address } => {
            send_query(adapter, &address, device::get_power_channel()).await
        }
        DeviceCmd::SetEnergy { address, enable } => {
            send_ok(
                adapter,
                &address,
                device::set_energy_ctrl(enable),
                &format!("Energy saving {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        DeviceCmd::GetEnergy { address } => {
            send_query(adapter, &address, device::get_energy_ctrl()).await
        }
        DeviceCmd::SetEyeGuard { address, enable } => {
            send_ok(
                adapter,
                &address,
                device::set_eye_guard(enable),
                &format!("Eye guard {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        DeviceCmd::GetEyeGuard { address } => {
            send_query(adapter, &address, device::get_eye_guard()).await
        }
        DeviceCmd::SetTempType { address, celsius } => {
            send_ok(
                adapter,
                &address,
                device::set_temp_type(celsius),
                &format!(
                    "Temp type: {}.",
                    if celsius { "Celsius" } else { "Fahrenheit" }
                ),
            )
            .await
        }
        DeviceCmd::GetTempType { address } => {
            send_query(adapter, &address, device::get_temp_type()).await
        }
        DeviceCmd::GetTemp { address } => {
            send_query(adapter, &address, device::get_device_temp()).await
        }
        DeviceCmd::GameKey { address, key } => {
            send_ok(adapter, &address, device::game_control(key), "Key sent.").await
        }
        DeviceCmd::GameKeyUp { address, key } => {
            send_ok(adapter, &address, device::game_key_up(key), "Key released.").await
        }
        DeviceCmd::SetGame {
            address,
            enable,
            param,
        } => {
            send_ok(
                adapter,
                &address,
                device::set_game(enable, param),
                "Game set.",
            )
            .await
        }
        DeviceCmd::GetToolInfo { address, tool_type } => {
            send_query(adapter, &address, device::get_tool_info(tool_type)).await
        }
        DeviceCmd::SetCountdown {
            address,
            enable,
            minutes,
            seconds,
        } => {
            send_ok(
                adapter,
                &address,
                device::set_tool_countdown(enable, minutes, seconds),
                "Countdown set.",
            )
            .await
        }
        DeviceCmd::SetNoise { address, param } => {
            send_ok(
                adapter,
                &address,
                device::set_tool_noise(param),
                "Noise meter set.",
            )
            .await
        }
        DeviceCmd::SetScoreboard {
            address,
            enable,
            score1,
            score2,
        } => {
            send_ok(
                adapter,
                &address,
                device::set_tool_scoreboard(enable, score1, score2),
                &format!("Scoreboard: {} - {}.", score1, score2),
            )
            .await
        }
        DeviceCmd::SetLanguage { address, lang } => {
            send_ok(
                adapter,
                &address,
                device::set_language(lang),
                "Language set.",
            )
            .await
        }
        DeviceCmd::SetAutoConnect { address, enable } => {
            send_ok(
                adapter,
                &address,
                device::set_auto_connect(enable),
                &format!("Auto-connect {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        DeviceCmd::GetAutoConnect { address } => {
            send_query(adapter, &address, device::get_auto_connect()).await
        }
        DeviceCmd::SetSaveVolume { address, enable } => {
            send_ok(
                adapter,
                &address,
                device::set_save_volume(enable),
                &format!("Save volume {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        DeviceCmd::SetPeripheral { address, enable } => {
            send_ok(
                adapter,
                &address,
                device::set_peripheral_ctrl(enable),
                "Peripheral set.",
            )
            .await
        }
        DeviceCmd::SetTalk {
            address,
            enable,
            param,
        } => {
            send_ok(
                adapter,
                &address,
                device::set_talk(enable, param),
                "Talk mode set.",
            )
            .await
        }
    }
}

// ---------------------------------------------------------------------------
// Display handler
// ---------------------------------------------------------------------------

async fn handle_display(adapter: &Adapter, cmd: DisplayCmd) -> Result<()> {
    use commands::display;
    match cmd {
        DisplayCmd::SetMode {
            address,
            mode,
            param,
        } => {
            let pkt = match mode.to_lowercase().as_str() {
                "clock" => display::set_box_mode_clock(),
                "temperature" | "temp" => display::set_box_mode_temperature(),
                "special" => display::set_box_mode_special(param),
                "sound" | "sound-reactive" => display::set_box_mode_sound_reactive(param),
                "music" => display::set_box_mode_music(param),
                other => anyhow::bail!(
                    "unknown mode '{}' (use: clock, temperature, special, sound, music)",
                    other
                ),
            };
            send_ok(
                adapter,
                &address,
                pkt,
                &format!("Display mode set to {}.", mode),
            )
            .await
        }
        DisplayCmd::GetMode { address } => {
            send_query(adapter, &address, display::get_box_mode()).await
        }
        DisplayCmd::SetColorLight {
            address,
            r,
            g,
            b,
            brightness,
            speed,
        } => {
            send_ok(
                adapter,
                &address,
                display::set_box_mode_color_light(r, g, b, brightness, speed),
                "Color light set.",
            )
            .await
        }
        DisplayCmd::GetScene { address } => {
            send_query(adapter, &address, display::get_scene()).await
        }
        DisplayCmd::TryColor { address, r, g, b } => {
            send_ok(
                adapter,
                &address,
                display::try_color(r, g, b),
                &format!("Color preview: ({}, {}, {}).", r, g, b),
            )
            .await
        }
        DisplayCmd::ChangeMode { address, mode } => {
            let wm = match mode.to_lowercase().as_str() {
                "bluetooth" | "bt" => display::WorkMode::Bluetooth,
                "linein" | "line-in" => display::WorkMode::LineIn,
                "sd" => display::WorkMode::Sd,
                "uac" | "usb" => display::WorkMode::Uac,
                other => anyhow::bail!(
                    "unknown work mode '{}' (use: bluetooth, linein, sd, uac)",
                    other
                ),
            };
            send_ok(
                adapter,
                &address,
                display::change_mode(wm),
                &format!("Work mode set to {}.", mode),
            )
            .await
        }
        DisplayCmd::SetScreenDir { address, direction } => {
            send_ok(
                adapter,
                &address,
                display::set_screen_direction(direction),
                "Screen direction set.",
            )
            .await
        }
        DisplayCmd::GetScreenDir { address } => {
            send_query(adapter, &address, display::get_screen_direction()).await
        }
        DisplayCmd::SetScreenMirror { address, enable } => {
            send_ok(
                adapter,
                &address,
                display::set_screen_mirror(enable),
                &format!("Screen mirror {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        DisplayCmd::SetScreenOn { address, on } => {
            send_ok(
                adapter,
                &address,
                display::set_screen_on(on),
                &format!("Screen {}.", if on { "on" } else { "off" }),
            )
            .await
        }
        DisplayCmd::SetLightArrow { address, mode } => {
            send_ok(
                adapter,
                &address,
                display::set_light_arrow(mode),
                "Light arrow set.",
            )
            .await
        }
        DisplayCmd::SetLightEffect { address, params } => {
            let bytes = parse_hex_or_bytes(&params)?;
            if bytes.len() != 7 {
                anyhow::bail!("need exactly 7 param bytes, got {}", bytes.len());
            }
            let arr: [u8; 7] = bytes.try_into().unwrap();
            send_ok(
                adapter,
                &address,
                display::set_light_effect(&arr),
                "Light effect set.",
            )
            .await
        }
        DisplayCmd::ResetLightEffect { address } => {
            send_ok(
                adapter,
                &address,
                display::reset_light_effect(),
                "Light effect reset.",
            )
            .await
        }
    }
}

// ---------------------------------------------------------------------------
// Alarm handler
// ---------------------------------------------------------------------------

async fn handle_alarm(adapter: &Adapter, cmd: AlarmCmd) -> Result<()> {
    use commands::alarm;
    match cmd {
        AlarmCmd::Get { address } => send_query(adapter, &address, alarm::get_alarms()).await,
        AlarmCmd::Set {
            address,
            id,
            enable,
            hour,
            minute,
            mode,
            weekdays,
            frequency,
            speed,
            volume,
        } => {
            send_ok(
                adapter,
                &address,
                alarm::set_alarm(
                    id, enable, hour, minute, mode, weekdays, frequency, speed, volume,
                ),
                &format!("Alarm {} set for {:02}:{:02}.", id, hour, minute),
            )
            .await
        }
        AlarmCmd::SetListen {
            address,
            enable,
            param1,
            param2,
        } => {
            send_ok(
                adapter,
                &address,
                alarm::set_alarm_listen(enable, param1, param2),
                "Alarm listen set.",
            )
            .await
        }
        AlarmCmd::SetListenVolume { address, volume } => {
            send_ok(
                adapter,
                &address,
                alarm::set_alarm_listen_volume(volume),
                &format!("Alarm volume set to {}.", volume),
            )
            .await
        }
        AlarmCmd::VoiceCtrl {
            address,
            control,
            volume,
        } => {
            let ctrl = match control {
                0 => alarm::AlarmVoiceControl::Record,
                1 => alarm::AlarmVoiceControl::Play,
                2 => alarm::AlarmVoiceControl::Stop,
                _ => anyhow::bail!("control must be 0 (record), 1 (play), or 2 (stop)"),
            };
            send_ok(
                adapter,
                &address,
                alarm::set_alarm_voice_ctrl(ctrl, volume),
                "Alarm voice control sent.",
            )
            .await
        }
        AlarmCmd::SetSleepTime { address, config } => {
            let bytes = parse_hex_or_bytes(&config)?;
            send_ok(
                adapter,
                &address,
                alarm::set_sleep_time(&bytes),
                "Sleep time set.",
            )
            .await
        }
        AlarmCmd::GetSleepMode { address } => {
            send_query(adapter, &address, alarm::get_sleep_mode()).await
        }
        AlarmCmd::SetSleepColor { address, r, g, b } => {
            send_ok(
                adapter,
                &address,
                alarm::set_sleep_color(r, g, b),
                "Sleep color set.",
            )
            .await
        }
        AlarmCmd::SetSleepLight { address, level } => {
            send_ok(
                adapter,
                &address,
                alarm::set_sleep_light(level),
                &format!("Sleep light set to {}.", level),
            )
            .await
        }
        AlarmCmd::SetAutoPowerOff { address, minutes } => {
            send_ok(
                adapter,
                &address,
                alarm::set_auto_power_off(minutes),
                &format!("Auto power-off set to {} min.", minutes),
            )
            .await
        }
        AlarmCmd::GetAutoPowerOff { address } => {
            send_query(adapter, &address, alarm::get_auto_power_off()).await
        }
    }
}

// ---------------------------------------------------------------------------
// Audio handler
// ---------------------------------------------------------------------------

async fn handle_audio(adapter: &Adapter, cmd: AudioCmd) -> Result<()> {
    use commands::audio;
    match cmd {
        AudioCmd::SetVolume { address, level } => {
            send_ok(
                adapter,
                &address,
                audio::set_volume(level),
                &format!("Volume set to {}.", level),
            )
            .await
        }
        AudioCmd::GetVolume { address } => send_query(adapter, &address, audio::get_volume()).await,
        AudioCmd::PlayPause { address } => {
            send_ok(
                adapter,
                &address,
                audio::play_pause(),
                "Play/pause toggled.",
            )
            .await
        }
        AudioCmd::Next { address } => {
            send_ok(adapter, &address, audio::skip_next(), "Next track.").await
        }
        AudioCmd::Prev { address } => {
            send_ok(adapter, &address, audio::skip_prev(), "Previous track.").await
        }
        AudioCmd::SdPlay { address, id } => {
            send_ok(
                adapter,
                &address,
                audio::set_sd_music_id(id),
                &format!("Playing SD track {}.", id),
            )
            .await
        }
        AudioCmd::SdSeek { address, pos } => {
            send_ok(
                adapter,
                &address,
                audio::set_sd_music_position(pos),
                "Seeked.",
            )
            .await
        }
        AudioCmd::SdPlayMode { address, mode } => {
            send_ok(
                adapter,
                &address,
                audio::set_sd_music_play_mode(mode),
                "SD play mode set.",
            )
            .await
        }
        AudioCmd::SdList {
            address,
            start,
            end,
        } => send_query(adapter, &address, audio::get_sd_music_list(start, end)).await,
        AudioCmd::SdInfo { address } => {
            send_query(adapter, &address, audio::get_sd_music_info()).await
        }
        AudioCmd::SetSoundCtrl { address, enable } => {
            send_ok(
                adapter,
                &address,
                audio::set_sound_ctrl(enable),
                &format!("Sound control {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        AudioCmd::GetSoundCtrl { address } => {
            send_query(adapter, &address, audio::get_sound_ctrl()).await
        }
        AudioCmd::SetEqualizer {
            address,
            mode,
            bands,
        } => {
            let vals = parse_hex_or_bytes(&bands)?;
            if vals.len() != 10 {
                anyhow::bail!("need exactly 10 band values, got {}", vals.len());
            }
            let arr: [u8; 10] = vals.try_into().unwrap();
            send_ok(
                adapter,
                &address,
                audio::set_equalizer(mode, &arr),
                "Equalizer set.",
            )
            .await
        }
        AudioCmd::ResetEqualizer { address } => {
            send_ok(
                adapter,
                &address,
                audio::reset_equalizer(),
                "Equalizer reset.",
            )
            .await
        }
        AudioCmd::Voice { address, control } => {
            send_ok(
                adapter,
                &address,
                audio::set_voice_playback(control),
                "Voice control sent.",
            )
            .await
        }
        AudioCmd::VoiceStatus { address } => {
            send_query(adapter, &address, audio::get_voice_status()).await
        }
        AudioCmd::SetMixer {
            address,
            mode,
            param,
        } => {
            send_ok(
                adapter,
                &address,
                audio::set_mixer_mode(mode, param),
                "Mixer set.",
            )
            .await
        }
        AudioCmd::SetMic { address, on } => {
            send_ok(
                adapter,
                &address,
                audio::set_mic_switch(on),
                &format!("Mic {}.", if on { "on" } else { "off" }),
            )
            .await
        }
        AudioCmd::Record { address, control } => {
            send_ok(
                adapter,
                &address,
                audio::record_ctrl(control),
                "Record control sent.",
            )
            .await
        }
        AudioCmd::SetStartupVolume { address, volume } => {
            send_ok(
                adapter,
                &address,
                audio::set_poweron_voice_volume(volume),
                &format!("Startup volume set to {}.", volume),
            )
            .await
        }
        AudioCmd::MuteStartup { address } => {
            send_ok(
                adapter,
                &address,
                audio::mute_poweron_voice(),
                "Startup sound muted.",
            )
            .await
        }
        AudioCmd::SetSongDisplay { address, enable } => {
            send_ok(
                adapter,
                &address,
                audio::set_song_display(enable),
                &format!("Song display {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        AudioCmd::GetSongDisplay { address } => {
            send_query(adapter, &address, audio::get_song_display()).await
        }
    }
}

// ---------------------------------------------------------------------------
// Drawing handler
// ---------------------------------------------------------------------------

async fn handle_drawing(adapter: &Adapter, cmd: DrawingCmd) -> Result<()> {
    use commands::drawing;
    match cmd {
        DrawingCmd::SendPic {
            address,
            pic_id,
            data,
        } => {
            let bytes = parse_hex_or_bytes(&data)?;
            send_ok(
                adapter,
                &address,
                drawing::encode_pic(pic_id, &bytes),
                "Picture sent.",
            )
            .await
        }
        DrawingCmd::PadEnter { address, r, g, b } => {
            send_ok(
                adapter,
                &address,
                drawing::drawing_pad_enter(r, g, b),
                "Drawing pad entered.",
            )
            .await
        }
        DrawingCmd::PadDraw {
            address,
            x,
            y,
            r,
            g,
            b,
        } => {
            send_ok(
                adapter,
                &address,
                drawing::drawing_pad_ctrl(x, y, r, g, b),
                &format!("Pixel ({}, {}) set.", x, y),
            )
            .await
        }
        DrawingCmd::PadExit { address } => {
            send_ok(
                adapter,
                &address,
                drawing::drawing_pad_exit(),
                "Drawing pad exited.",
            )
            .await
        }
        DrawingCmd::MoviePlay { address, play } => {
            send_ok(
                adapter,
                &address,
                drawing::ctrl_movie_play(play),
                &format!("Movie {}.", if play { "playing" } else { "stopped" }),
            )
            .await
        }
        DrawingCmd::GifStart { address, anim_id } => {
            send_ok(
                adapter,
                &address,
                drawing::set_user_gif_start(anim_id),
                "GIF started.",
            )
            .await
        }
        DrawingCmd::GifEnd { address } => {
            send_ok(adapter, &address, drawing::set_user_gif_end(), "GIF ended.").await
        }
        DrawingCmd::GifStop { address } => {
            send_ok(
                adapter,
                &address,
                drawing::stop_send_gif(),
                "GIF send stopped.",
            )
            .await
        }
        DrawingCmd::SetGifSpeed { address, speed } => {
            send_ok(
                adapter,
                &address,
                drawing::set_gif_speed(speed),
                &format!("GIF speed set to {}.", speed),
            )
            .await
        }
        DrawingCmd::SetGifPlayTime { address, time } => {
            send_ok(
                adapter,
                &address,
                drawing::set_gif_play_time(time),
                &format!("GIF play time set to {}.", time),
            )
            .await
        }
        DrawingCmd::ResetGifPlayTime { address } => {
            send_ok(
                adapter,
                &address,
                drawing::reset_gif_play_time(),
                "GIF play time reset.",
            )
            .await
        }
        DrawingCmd::SandPaintStart { address } => {
            send_ok(
                adapter,
                &address,
                drawing::sand_paint_start(),
                "Sand paint started.",
            )
            .await
        }
        DrawingCmd::Scroll {
            address,
            mode,
            speed,
        } => {
            send_ok(
                adapter,
                &address,
                drawing::scroll(mode, speed),
                "Scroll set.",
            )
            .await
        }
        DrawingCmd::SetBootGif { address, enable } => {
            send_ok(
                adapter,
                &address,
                drawing::set_boot_gif(enable),
                &format!("Boot animation {}.", if enable { "on" } else { "off" }),
            )
            .await
        }
        DrawingCmd::SendGifStart { address } => {
            send_ok(
                adapter,
                &address,
                drawing::send_gif_start(),
                "GIF start signal sent.",
            )
            .await
        }
    }
}

// ---------------------------------------------------------------------------
// Text handler
// ---------------------------------------------------------------------------

async fn handle_text(adapter: &Adapter, cmd: TextCmd) -> Result<()> {
    use commands::text;
    match cmd {
        TextCmd::Send { address, text: t } => {
            send_ok(
                adapter,
                &address,
                text::send_led_text(&t),
                &format!("Text sent: '{}'.", t),
            )
            .await
        }
        TextCmd::Clear { address } => {
            send_ok(adapter, &address, text::clear_led_text(), "Text cleared.").await
        }
        TextCmd::SetColor { address, r, g, b } => {
            send_ok(
                adapter,
                &address,
                text::set_text_color(r, g, b),
                "Text color set.",
            )
            .await
        }
        TextCmd::SetEffect { address, effect } => {
            send_ok(
                adapter,
                &address,
                text::set_text_effect(effect),
                "Text effect set.",
            )
            .await
        }
        TextCmd::SetSize { address, size } => {
            send_ok(
                adapter,
                &address,
                text::set_text_size(size),
                "Text size set.",
            )
            .await
        }
        TextCmd::SetSpeed { address, speed } => {
            send_ok(
                adapter,
                &address,
                text::set_text_speed(speed),
                &format!("Text speed set to {}.", speed),
            )
            .await
        }
        TextCmd::SetNotifyPic {
            address,
            id,
            r,
            g,
            b,
        } => {
            send_ok(
                adapter,
                &address,
                text::set_notification_pic(id, r, g, b),
                "Notification picture set.",
            )
            .await
        }
        TextCmd::SetNotifyText {
            address,
            notify_type,
            text: t,
        } => {
            send_ok(
                adapter,
                &address,
                text::set_notification_text(notify_type, &t),
                "Notification text sent.",
            )
            .await
        }
        TextCmd::ResetNotify { address } => {
            send_ok(
                adapter,
                &address,
                text::reset_notifications(),
                "Notifications reset.",
            )
            .await
        }
        TextCmd::DeleteLeaveMsg { address } => {
            send_ok(
                adapter,
                &address,
                text::delete_leave_message(),
                "Leave message deleted.",
            )
            .await
        }
        TextCmd::ScrollText {
            address,
            text: t,
            speed,
        } => {
            let ch = connect(adapter, &address).await?;
            for pkt in text::set_scroll_text(&t, speed) {
                send(&ch, &pkt).await?;
            }
            println!("Scrolling text sent: '{}'.", t);
            Ok(())
        }
        TextCmd::GetDailyTime { address } => {
            send_query(adapter, &address, text::get_daily_time()).await
        }
    }
}
