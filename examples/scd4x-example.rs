// Based on https://github.com/hauju/scd4x-rs/blob/b9b0cb774b78ec7317c3e66f2fae745711101abb/examples/linux.rs

use embedded_hal::delay::DelayNs;
use rtt_target::rprintln;

use scd4x::Scd4x;

fn main() {
    let mut sensor = Scd4x::new(i2c, timer);

    sensor.start_periodic_measurement().unwrap();
    loop {
        timer.delay_ms(5000u32);

        let data = sensor.measurement().unwrap();

        rprintln!(
            "CO2: {0}, Temperature: {1:#.2} °C, Humidity: {2:#.2} RH",
            data.co2,
            data.temperature,
            data.humidity
        );
    }
}
