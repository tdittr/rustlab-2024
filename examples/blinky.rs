async fn main() {
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        for i in 1..=1000 {
            led.set_high().unwrap();
            Timer::after_millis(i).await;
            led.set_low()?;
            Timer::after_millis(i).await;
        }
    }
}
