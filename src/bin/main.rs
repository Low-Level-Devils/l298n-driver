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
use esp_hal::ledc::*;
use esp_hal::main;
use l298n_driver::l298n_control::{self, L298n};
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

    let l298n_ledc = l298n_control::initialize_ledc(peripherals.LEDC);
    let l298n_lstimer = l298n_control::initialize_lstimer(&l298n_ledc);

    let l298n_module = L298n::new(
        &l298n_ledc,
        &l298n_lstimer,
        channel::Number::Channel0,
        peripherals.GPIO2.into(),
        channel::Number::Channel1,
        peripherals.GPIO4.into(),
    );

    loop {
        info!("Motor Forward Full");
        l298n_module.change_speed(100);
        delay.delay_millis(2000);
        info!("Motor reverse Full");
        l298n_module.change_speed(-100);
        delay.delay_millis(2000);
        info!("Motor Forward 25%");
        l298n_module.change_speed(25);
        delay.delay_millis(2000);
        info!("Motor Forward 50%");
        l298n_module.change_speed(50);
        delay.delay_millis(2000);
        info!("Motor Forward 75%");
        l298n_module.change_speed(75);
        delay.delay_millis(2000);
        info!("Motor Reverse 25%");
        l298n_module.change_speed(-25);
        delay.delay_millis(2000);
        info!("Motor Reverse 50%");
        l298n_module.change_speed(-50);
        delay.delay_millis(2000);
        info!("Motor Reverse 75%");
        l298n_module.change_speed(-75);
        delay.delay_millis(2000);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
