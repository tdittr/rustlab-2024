//! Measure CO2, make LED color
#![no_std]
#![no_main]

use adafruit_metro_rp2040::{
    entry,
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        fugit::RateExtU32,
        pac,
        pio::PIOExt,
        watchdog::Watchdog,
        Sio, Timer, I2C,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use core::iter::once;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use scd4x::Scd4x;
use smart_leds::{brightness, colors, SmartLedsWrite};
use ws2812_pio::Ws2812;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led = pins.neopixel_data.into_function();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        led,
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut timer = timer; // rebind to force a copy of the timer

    let i2c = I2C::i2c0(
        pac.I2C0,
        pins.sda.reconfigure(),
        pins.scl.reconfigure(),
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    let mut scd40 = Scd4x::new(i2c, timer);
    scd40.stop_periodic_measurement().unwrap();
    scd40.reinit().unwrap();

    let serial = scd40.serial_number().unwrap();
    rprintln!("serial: {:#04x}", serial);

    scd40.start_periodic_measurement().unwrap();

    let mut led_pin = pins.d13.into_push_pull_output();

    loop {
        timer.delay_ms(5000);

        led_pin.set_high().unwrap();
        let meas = scd40.measurement().unwrap();
        led_pin.set_low().unwrap();

        rprintln!(
            "CO2: {0}ppm, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
            meas.co2,
            meas.temperature,
            meas.humidity
        );

        let color = match meas.co2 {
            x if x < 400 => colors::GREEN,
            x if x < 1000 => colors::YELLOW,
            _ => colors::RED,
        };

        ws.write(brightness(once(color), 64)).unwrap();
    }
}
