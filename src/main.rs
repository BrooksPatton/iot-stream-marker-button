use gpio::{GpioIn, GpioOut};
use std::{thread, time};

fn main() {
    let mut gpio14 = gpio::sysfs::SysFsGpioInput::open(8).unwrap();

    loop {
        println!("GPIO14: {:?}", gpio14.read_value().unwrap());
        thread::sleep(time::Duration::from_millis(100));
    }
}
