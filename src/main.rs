use gpio::{GpioIn, GpioOut};
use std::{thread, time};

fn main() {
    let mut gpio2 = gpio::sysfs::SysFsGpioInput::open(2).unwrap();
    let mut gpio3 = gpio::sysfs::SysFsGpioInput::open(3).unwrap();
    let mut gpio4 = gpio::sysfs::SysFsGpioInput::open(4).unwrap();
    let mut gpio17 = gpio::sysfs::SysFsGpioInput::open(17).unwrap();
    let mut gpio27 = gpio::sysfs::SysFsGpioInput::open(27).unwrap();
    let mut gpio22 = gpio::sysfs::SysFsGpioInput::open(22).unwrap();
    let mut gpio10 = gpio::sysfs::SysFsGpioInput::open(10).unwrap();
    let mut gpio9 = gpio::sysfs::SysFsGpioInput::open(9).unwrap();
    let mut gpio11 = gpio::sysfs::SysFsGpioInput::open(11).unwrap();
    let mut gpio0 = gpio::sysfs::SysFsGpioInput::open(0).unwrap();
    let mut gpio5 = gpio::sysfs::SysFsGpioInput::open(5).unwrap();
    let mut gpio6 = gpio::sysfs::SysFsGpioInput::open(6).unwrap();
    let mut gpio13 = gpio::sysfs::SysFsGpioInput::open(13).unwrap();
    let mut gpio19 = gpio::sysfs::SysFsGpioInput::open(19).unwrap();
    let mut gpio26 = gpio::sysfs::SysFsGpioInput::open(26).unwrap();
    let mut gpio14 = gpio::sysfs::SysFsGpioInput::open(14).unwrap();
    let mut gpio15 = gpio::sysfs::SysFsGpioInput::open(15).unwrap();
    let mut gpio18 = gpio::sysfs::SysFsGpioInput::open(18).unwrap();
    let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
    let mut gpio24 = gpio::sysfs::SysFsGpioInput::open(24).unwrap();
    let mut gpio25 = gpio::sysfs::SysFsGpioInput::open(25).unwrap();
    let mut gpio8 = gpio::sysfs::SysFsGpioInput::open(8).unwrap();
    let mut gpio7 = gpio::sysfs::SysFsGpioInput::open(7).unwrap();
    let mut gpio1 = gpio::sysfs::SysFsGpioInput::open(1).unwrap();
    let mut gpio12 = gpio::sysfs::SysFsGpioInput::open(12).unwrap();
    let mut gpio16 = gpio::sysfs::SysFsGpioInput::open(16).unwrap();
    let mut gpio20 = gpio::sysfs::SysFsGpioInput::open(20).unwrap();
    let mut gpio21 = gpio::sysfs::SysFsGpioInput::open(21).unwrap();

    loop {
        println!("gpio2: {:?}", gpio2.read_value().unwrap());
        println!("gpio3: {:?}", gpio3.read_value().unwrap());
        println!("gpio4: {:?}", gpio4.read_value().unwrap());
        println!("Ggpio17: {:?}", gpio17.read_value().unwrap());
        println!("Ggpio27: {:?}", gpio27.read_value().unwrap());
        println!("Ggpio22: {:?}", gpio22.read_value().unwrap());
        println!("Ggpio10: {:?}", gpio10.read_value().unwrap());
        println!("gpio9: {:?}", gpio9.read_value().unwrap());
        println!("Ggpio11: {:?}", gpio11.read_value().unwrap());
        println!("gpio0: {:?}", gpio0.read_value().unwrap());
        println!("gpio5: {:?}", gpio5.read_value().unwrap());
        println!("gpio6: {:?}", gpio6.read_value().unwrap());
        println!("Ggpio13: {:?}", gpio13.read_value().unwrap());
        println!("Ggpio19: {:?}", gpio19.read_value().unwrap());
        println!("Ggpio26: {:?}", gpio26.read_value().unwrap());
        println!("Ggpio14: {:?}", gpio14.read_value().unwrap());
        println!("Ggpio15: {:?}", gpio15.read_value().unwrap());
        println!("Ggpio18: {:?}", gpio18.read_value().unwrap());
        println!("Ggpio23: {:?}", gpio23.read_value().unwrap());
        println!("Ggpio24: {:?}", gpio24.read_value().unwrap());
        println!("Ggpio25: {:?}", gpio25.read_value().unwrap());
        println!("gpio8: {:?}", gpio8.read_value().unwrap());
        println!("gpio7: {:?}", gpio7.read_value().unwrap());
        println!("gpio1: {:?}", gpio1.read_value().unwrap());
        println!("Ggpio12: {:?}", gpio12.read_value().unwrap());
        println!("Ggpio16: {:?}", gpio16.read_value().unwrap());
        println!("Ggpio20: {:?}", gpio20.read_value().unwrap());
        println!("Ggpio21: {:?}", gpio21.read_value().unwrap());

        thread::sleep(time::Duration::from_millis(100));
    }
}
