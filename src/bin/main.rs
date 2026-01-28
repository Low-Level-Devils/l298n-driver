#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::*;
use esp_hal::ledc::channel::{ChannelHW, ChannelIFace};
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::*;
use esp_hal::main;
use esp_hal::time::Rate;
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    info!("initialized ledc");

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_hz(10000),
        })
        .expect("Error configuring lstimer");

    info!("initialized lstimer");

    let mut motor_anode_ledc_channel = ledc.channel(channel::Number::Channel0, peripherals.GPIO2);
    motor_anode_ledc_channel
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .expect("Fail to configure motor anode channel");

    info!("initialized motor anode ledc channel");

    let mut motor_cathode_ledc_channel = ledc.channel(channel::Number::Channel1, peripherals.GPIO4);
    motor_cathode_ledc_channel
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .expect("Fail to configure motor cathode channel");

    info!("initialized motor cathode ledc channel");

    loop {
        info!("Motor Forward Full");
        motor_anode_ledc_channel.set_duty_hw(4095);
        motor_cathode_ledc_channel.set_duty_hw(0);
        delay.delay_millis(2000);
        info!("Motor reverse Full");
        motor_anode_ledc_channel.set_duty_hw(0);
        motor_cathode_ledc_channel.set_duty_hw(4095);
        delay.delay_millis(2000);
        info!("Motor Forward 25%");
        motor_anode_ledc_channel.set_duty_hw(1000);
        motor_cathode_ledc_channel.set_duty_hw(0);
        delay.delay_millis(2000);
        info!("Motor Forward 50%");
        motor_anode_ledc_channel.set_duty_hw(2000);
        motor_cathode_ledc_channel.set_duty_hw(0);
        delay.delay_millis(2000);
        info!("Motor Forward 75%");
        motor_anode_ledc_channel.set_duty_hw(3000);
        motor_cathode_ledc_channel.set_duty_hw(0);
        delay.delay_millis(2000);
        info!("Motor Reverse 25%");
        motor_anode_ledc_channel.set_duty_hw(0);
        motor_cathode_ledc_channel.set_duty_hw(1000);
        delay.delay_millis(2000);
        info!("Motor Reverse 50%");
        motor_anode_ledc_channel.set_duty_hw(0);
        motor_cathode_ledc_channel.set_duty_hw(2000);
        delay.delay_millis(2000);
        info!("Motor Reverse 75%");
        motor_anode_ledc_channel.set_duty_hw(0);
        motor_cathode_ledc_channel.set_duty_hw(3000);
        delay.delay_millis(2000);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
