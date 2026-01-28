# Servo Driver

## Project Description
This Rust based driver is for controlling DC motors via L298N Module 

## Required Hardware
- ESP32
- DC Motor x2
- 12V DC power Supply
- Breadboard
- Jumper Wires

## Connections
> [!NOTE]
> This setup assumes the user wants the motors in sync. For asyncronous operations use different GPIO
> pins for motor 2.

### ESP32
| ESP32 Pin | Connection |
| :------: | :------: |
| GND | 12V Power Supply GND |
| GPIO 2 | L298N IN1 / IN3 |
| GPIO 4 | L298N IN2 / IN4 |

### DC Motor 1
| Motor Pin | Connection |
| :------: | :------: |
| Pin 1 | L298N OUT1 |
| Pin 2 | L298N OUT2 |

### DC Motor 2
| Motor Pin | Connection |
| :------: | :------: |
| Pin 1 | L298N OUT3 |
| Pin 2 | L298N OUT4 |

### L298N
| Motor Pin | Connection |
| :------: | :------: |
| +12V | Power Supply 12V |
| GND | Power Supply GND |
| IN1 | ESP32 GPIO 2 |
| IN2 | ESP32 GPIO 4 |
| IN3 | ESP32 GPIO 2 |
| IN3 | ESP32 GPIO 4 |
| OUT1 | Motor 1 Pin 1 |
| OUT2 | Motor 1 Pin 2 |
| OUT3 | Motor 2 Pin 1 |
| OUT4 | Motor 2 Pin 2 |

## Build and run
Connect the ESP32 to the host then run the following command to build, run, and monitor
```bash
cargo run --release
```