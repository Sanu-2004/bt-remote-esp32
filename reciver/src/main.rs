use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, PeripheralProperties};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
use std::str;
use std::time::Duration;
use tokio;
use std::process::Command;

const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0x4fafc201_1fb5_459e_8fcc_c5c9c331914b);
const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xbeb5483e_36e1_4688_b7f5_ea07361b26a8);
const ESP32_NAME: &str = "ESP32_Remote";
const CONNECTION_CHECK_INTERVAL: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        match connect_and_handle().await {
            Ok(_) => println!("Connection closed. Reconnecting..."),
            Err(e) => eprintln!("Error: {}. Retrying in 5 seconds...", e),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn connect_and_handle() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().next().ok_or("No Bluetooth adapters found")?;

    central.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;

    let peripherals = central.peripherals().await?;

    let peripheral = peripherals
        .into_iter()
        .find(|p: &Peripheral| {
            if let Ok(properties_option) = futures::executor::block_on(p.properties()) as Result<Option<PeripheralProperties>, btleplug::Error> {
                if let Some(properties) = properties_option {
                    if let Some(local_name) = properties.local_name {
                        return local_name == ESP32_NAME;
                    }
                }
            }
            false
        })
        .ok_or(format!("{} not found", ESP32_NAME).as_str())?;

    if !peripheral.is_connected().await? {
        peripheral.connect().await?;
    }

    peripheral.discover_services().await?;

    let service = peripheral
        .services()
        .into_iter()
        .find(|s| s.uuid == SERVICE_UUID)
        .ok_or("Service not found")?;

    let characteristic = service
        .characteristics
        .into_iter()
        .find(|c| c.uuid == CHARACTERISTIC_UUID)
        .ok_or("Characteristic not found")?;

    peripheral.subscribe(&characteristic).await?;

    let mut notification_stream = peripheral.notifications().await?;

    loop {
        tokio::select! {
            Some(data) = notification_stream.next() => {
                if data.uuid == CHARACTERISTIC_UUID {
                    if let Ok(value) = str::from_utf8(&data.value) {
                        if let Ok(code) = value.trim().parse::<i32>() {
                            println!("Received code: {}", code);
                            run_command(code);
                        }
                    }
                }
            }
            _ = tokio::time::sleep(CONNECTION_CHECK_INTERVAL) => {
                if !peripheral.is_connected().await? {
                    println!("Connection lost. Attempting to reconnect...");
                    return Ok(());
                }
            }
        }
    }
}

fn run_command(code: i32) {
    match code {
        100 => run_command_os("volumeup"),
        150 => run_command_os("volumedown"),
        200 => run_command_os("nexttrack"),
        250 => run_command_os("previoustrack"),
        300 => run_command_os("playpause"),
        350 => run_command_os("enter"),
        400 => run_command_os("altf4"),
        _ => println!("Unknown code: {}", code),
    }
}

fn run_command_os(command: &str) {
    match command {
        "volumeup" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys([char]175)"])
                .spawn()
                .expect("Failed to execute volume up command");
        }
        "volumedown" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys([char]174)"])
                .spawn()
                .expect("Failed to execute volume down command");
        }
        "nexttrack" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys([char]176)"])
                .spawn()
                .expect("Failed to execute next track command");
        }
        "previoustrack" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys([char]177)"])
                .spawn()
                .expect("Failed to execute previous track command");
        }
        "playpause" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys([char]179)"])
                .spawn()
                .expect("Failed to execute play/pause command");
        }
        "enter" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys('~')"])
                .spawn()
                .expect("Failed to execute enter command");
        }
        "altf4" => {
            Command::new("powershell")
                .args(&["-Command", "(new-object -com wscript.shell).SendKeys('%{F4}')"])
                .spawn()
                .expect("Failed to execute alt-f4 command");
        }
        _ => println!("Unknown command: {}", command),
    }
}