# ESP32 Remote Control Project

This project consists of an ESP32-based remote control and a Rust-based receiver application. The ESP32 sends touch sensor data via Bluetooth Low Energy (BLE) to the receiver, which then executes corresponding commands on the host machine.

## Project Structure

- `esp32_remote/`: Contains the ESP32 firmware code.
    - `ci.json`: Configuration file for the ESP32 build.
    - `esp32_remote.ino`: Arduino sketch for the ESP32 remote control.

- `reciver/`: Contains the Rust-based receiver application.
    - `src/main.rs`: Main source code for the receiver.
    - `Cargo.toml`: Configuration file for the Rust project.

## ESP32 Firmware

The ESP32 firmware uses the Arduino framework to read touch sensor inputs and send corresponding codes via BLE. The firmware includes:

- BLE initialization and advertising.
- Touch sensor reading and threshold detection.
- Power button handling to turn the device on and off.

### Dependencies

- `BLEDevice.h`
- `BLEServer.h`
- `BLEUtils.h`
- `BLE2902.h`
- `driver/touch_sensor.h`

### Setup

1. Install the required libraries in the Arduino IDE.
2. Upload the `esp32_remote.ino` sketch to your ESP32 board.

## Rust Receiver

The Rust receiver application connects to the ESP32 via BLE, receives touch sensor codes, and executes corresponding commands on the host machine.

### Dependencies

- `btleplug`: For Bluetooth communication.
- `tokio`: For async runtime.
- `futures`: For async/await support.
- `uuid`: For UUID handling.

### Setup

1. Install Rust and Cargo.
2. Navigate to the `reciver` directory.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the receiver application.

## Commands

The receiver application maps received codes to system commands:

- `100`: Volume Up
- `150`: Volume Down
- `200`: Next Track
- `250`: Previous Track
- `300`: Play/Pause
- `350`: Enter
- `400`: Alt+F4

## Usage

1. Power on the ESP32 remote.
2. Run the Rust receiver application on your host machine.
3. Touch the sensors on the ESP32 to send commands to the host machine.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

- [btleplug](https://github.com/deviceplug/btleplug)
- [Arduino](https://www.arduino.cc/)
