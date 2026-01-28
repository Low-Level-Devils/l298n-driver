#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::{main, peripherals};
use esp_hal::time::{Duration, Instant, Rate};
use log::info;
use esp_hal::ledc::*;
use esp_hal::gpio::*;
use esp_hal::delay::Delay;

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

    //let output_config = OutputConfig::default();
    //let mut motor_anode = Output::new(peripherals.GPIO2, Level::Low, output_config);
    //let mut motor_cathode = Output::new(peripherals.GPIO4, Level::Low, output_config);

    let mut delay = Delay::new();

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0.configure(timer::config::Config {
        duty: timer::config::Duty::Duty5Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: Rate::from_hz(24),
    }).expect("Error configuring lstimer");

    let mut motor_anode_ledc_channel = ledc.channel(channel::Number::Channel0, peripherals.GPIO2);
    motor_anode_ledc_channel.configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0,
        drive_mode: DriveMode::PushPull
    }).expect("Fail to configure motor anode channel");

    let mut motor_cathode_ledc_channel = ledc.channel(channel::Number::Channel0, peripherals.GPIO4);
    motor_cathode_ledc_channel.configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0,
        drive_mode: DriveMode::PushPull
    }).expect("Fail to configure motor cathode channel");

    loop {
        info!("Motor Forward Full");
        motor_anode_ledc_channel.set_duty(100).expect("Failed to set duty anode 1");
        motor_cathode_ledc_channel.set_duty(0).expect("Failed to set duty cathode 1");
        delay.delay_millis(2000);
        info!("Motor reverse Full");
        motor_anode_ledc_channel.set_duty(0).expect("Failed to set duty anode 2");
        motor_cathode_ledc_channel.set_duty(100).expect("Failed to set duty cathode 2");
        delay.delay_millis(2000);
        info!("Motor Forward 25%");
        motor_anode_ledc_channel.set_duty(25).expect("Failed to set duty anode 3");
        motor_cathode_ledc_channel.set_duty(0).expect("Failed to set duty cathode 3");
        delay.delay_millis(2000);
        info!("Motor Forward 50%");
        motor_anode_ledc_channel.set_duty(50).expect("Failed to set duty anode 4");
        motor_cathode_ledc_channel.set_duty(0).expect("Failed to set duty cathode 4");
        delay.delay_millis(2000);
        info!("Motor Forward 75%");
        motor_anode_ledc_channel.set_duty(75).expect("Failed to set duty anode 5");
        motor_cathode_ledc_channel.set_duty(0).expect("Failed to set duty cathode 5");
        delay.delay_millis(2000);
        info!("Motor Reverse 25%");
        motor_anode_ledc_channel.set_duty(0).expect("Failed to set duty anode 6");
        motor_cathode_ledc_channel.set_duty(25).expect("Failed to set duty cathode 6");
        delay.delay_millis(2000);
        info!("Motor Reverse 50%");
        motor_anode_ledc_channel.set_duty(0).expect("Failed to set duty anode 7");
        motor_cathode_ledc_channel.set_duty(50).expect("Failed to set duty cathode 7");
        delay.delay_millis(2000);
        info!("Motor Reverse 75%");
        motor_anode_ledc_channel.set_duty(0).expect("Failed to set duty anode 8");
        motor_cathode_ledc_channel.set_duty(75).expect("Failed to set duty cathode 8");
        delay.delay_millis(2000);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
