mod commands;
mod protocol;

use anyhow::{Context, Result};
use bluer::{gatt::remote::Characteristic, Adapter, Address, Device};
use clap::{Parser, Subcommand};
use protocol::Packet;

const DIVOOM_CHARACTERISTIC: bluer::Uuid =
    bluer::Uuid::from_u128(0x49535343_1e4d_4bd9_ba61_23c647249616);

#[derive(Parser)]
#[command(name = "pixelbox", about = "Control Divoom Ditoo Pro over BLE")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for nearby Divoom devices
    Scan {
        /// Scan duration in seconds
        #[arg(short, long, default_value = "5")]
        duration: u64,
    },
    /// Set the device clock to the current system time
    SetTime {
        /// BLE MAC address of the device (e.g. AA:BB:CC:DD:EE:FF)
        #[arg(short, long)]
        address: String,
    },
    /// Set display brightness
    SetBrightness {
        /// BLE MAC address of the device
        #[arg(short, long)]
        address: String,
        /// Brightness level (0-100)
        #[arg(short, long)]
        level: u8,
    },
    /// Query device info
    Info {
        /// BLE MAC address of the device
        #[arg(short, long)]
        address: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    match cli.command {
        Commands::Scan { duration } => cmd_scan(&adapter, duration).await?,
        Commands::SetTime { address } => {
            let addr = parse_address(&address)?;
            let char = connect_and_find_char(&adapter, addr).await?;
            cmd_set_time(&char).await?;
        }
        Commands::SetBrightness { address, level } => {
            let addr = parse_address(&address)?;
            let char = connect_and_find_char(&adapter, addr).await?;
            cmd_set_brightness(&char, level).await?;
        }
        Commands::Info { address } => {
            let addr = parse_address(&address)?;
            let device = adapter.device(addr)?;
            cmd_info(&device).await?;
        }
    }

    Ok(())
}

fn parse_address(s: &str) -> Result<Address> {
    s.parse::<Address>()
        .with_context(|| format!("invalid BLE address: {s}"))
}

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
                if let AdapterEvent::DeviceAdded(addr) = event {
                    if let Ok(device) = adapter.device(addr) {
                        let name = device.name().await.ok().flatten();
                        if let Some(ref n) = name {
                            if n.to_lowercase().contains("ditoo")
                                || n.to_lowercase().contains("divoom")
                                || n.to_lowercase().contains("timebox")
                                || n.to_lowercase().contains("tivoo")
                                || n.to_lowercase().contains("pixoo")
                            {
                                println!("  [DIVOOM] {} - {}", addr, n);
                            } else {
                                println!("          {} - {}", addr, n);
                            }
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

async fn connect_and_find_char(
    adapter: &Adapter,
    addr: Address,
) -> Result<Characteristic> {
    let device = adapter.device(addr)?;

    if !device.is_connected().await? {
        println!("Connecting to {}...", addr);
        device.connect().await.context("failed to connect")?;
        println!("Connected.");
    }

    // Give the device a moment to expose services
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("Discovering services...");
    for service in device.services().await? {
        for char in service.characteristics().await? {
            if char.uuid().await? == DIVOOM_CHARACTERISTIC {
                println!("Found Divoom characteristic.");
                return Ok(char);
            }
        }
    }

    anyhow::bail!(
        "Divoom characteristic {} not found on device {}",
        DIVOOM_CHARACTERISTIC,
        addr
    );
}

async fn send_packet(char: &Characteristic, packet: &Packet) -> Result<()> {
    let bytes = packet.encode();
    // BLE writes must be chunked to 20 bytes (Divoom protocol requirement)
    for chunk in bytes.chunks(20) {
        char.write(chunk).await.context("BLE write failed")?;
    }
    Ok(())
}

async fn cmd_set_time(char: &Characteristic) -> Result<()> {
    let packet = protocol::set_time_now();
    send_packet(char, &packet).await?;
    println!("Time set to current system time.");
    Ok(())
}

async fn cmd_set_brightness(char: &Characteristic, level: u8) -> Result<()> {
    let level = level.min(100);
    let packet = protocol::set_brightness(level);
    send_packet(char, &packet).await?;
    println!("Brightness set to {}%.", level);
    Ok(())
}

async fn cmd_info(device: &Device) -> Result<()> {
    let name = device.name().await.ok().flatten().unwrap_or_default();
    let addr = device.address();
    let connected = device.is_connected().await.unwrap_or(false);
    let rssi = device.rssi().await.ok().flatten();

    println!("Device: {}", name);
    println!("Address: {}", addr);
    println!("Connected: {}", connected);
    if let Some(r) = rssi {
        println!("RSSI: {} dBm", r);
    }

    Ok(())
}
