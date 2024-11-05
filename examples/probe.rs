use rtt_target::{rprintln, rtt_init_print};

fn main() -> ! {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");
    }
}
