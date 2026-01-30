#![no_std]

pub mod l298n_control {
    use esp_hal::gpio::*;
    use esp_hal::ledc::channel::{Channel, ChannelHW, ChannelIFace};
    use esp_hal::ledc::timer::Timer;
    use esp_hal::ledc::timer::TimerIFace;
    use esp_hal::ledc::*;
    use esp_hal::peripherals::LEDC;
    use esp_hal::time::Rate;

    pub fn initialize_ledc<'d>(ledc_peripheral: LEDC<'d>) -> Ledc<'d> {
        let mut ledc = Ledc::new(ledc_peripheral);
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

        ledc
    }

    pub fn initialize_lstimer<'d>(ledc: &Ledc<'d>) -> Timer<'d, LowSpeed> {
        let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
        lstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty12Bit,
                clock_source: timer::LSClockSource::APBClk,
                frequency: Rate::from_hz(10000),
            })
            .expect("Error configuring lstimer");

        lstimer0
    }

    pub struct L298n<'d, 't> {
        motor_anode_ledc_channel: Channel<'d, LowSpeed>,
        motor_cathode_ledc_channel: Channel<'d, LowSpeed>,
        _timer: &'t Timer<'d, LowSpeed>,
    }

    impl<'d, 't: 'd> L298n<'d, 't> {
        pub fn new(
            ledc: &Ledc<'d>,
            timer: &'t Timer<'d, LowSpeed>,
            anode_channel: channel::Number,
            anode_pin: AnyPin<'d>,
            cathode_channel: channel::Number,
            cathode_pin: AnyPin<'d>,
        ) -> Self {
            let mut motor_anode_ledc_channel = ledc.channel(anode_channel, anode_pin);
            motor_anode_ledc_channel
                .configure(channel::config::Config {
                    timer: timer,
                    duty_pct: 0,
                    drive_mode: DriveMode::PushPull,
                })
                .expect("Fail to configure motor anode channel");

            let mut motor_cathode_ledc_channel = ledc.channel(cathode_channel, cathode_pin);
            motor_cathode_ledc_channel
                .configure(channel::config::Config {
                    timer: timer,
                    duty_pct: 0,
                    drive_mode: DriveMode::PushPull,
                })
                .expect("Fail to configure motor cathode channel");

            Self {
                motor_anode_ledc_channel,
                motor_cathode_ledc_channel,
                _timer: timer,
            }
        }

        fn calculate_duty(&self, percent: i32) -> u32 {
            percent.abs() as u32 * 4095 / 100
        }

        pub fn change_speed(&self, percent: i32) {
            let duty = self.calculate_duty(percent);

            if percent >= 0 {
                self.motor_anode_ledc_channel.set_duty_hw(duty);
                self.motor_cathode_ledc_channel.set_duty_hw(0);
            } else {
                self.motor_anode_ledc_channel.set_duty_hw(0);
                self.motor_cathode_ledc_channel.set_duty_hw(duty);
            }
        }
    }
}
