mod gpio_wrapper;
use std::{thread, time::Duration};

use gpio_wrapper::GpioWrapper;


fn main() {
    let unit = 2000;
    let r1: u8 = 17;
    let y1: u8 = 5;
    let g1: u8 = 26;

    let r2: u8 = 18;
    let y2: u8 = 24;
    let g2: u8 = 12;

    let mut gw = GpioWrapper::new(r1, y1, g1, r2, y2, g2);
    gw.r1_on();
    gw.r2_on();
    thread::sleep(Duration::from_millis(unit));

    loop {
        gw.r1_off();
        gw.y1_on();
        thread::sleep(Duration::from_millis(unit));

        gw.y1_off();
        gw.g1_on();
        thread::sleep(Duration::from_millis(unit * 5));

        gw.g1_off();
        gw.y1_on();
        thread::sleep(Duration::from_millis(unit));

        gw.y1_off();
        gw.r1_on();
        thread::sleep(Duration::from_millis(unit));

        gw.r2_off();
        gw.y2_on();
        thread::sleep(Duration::from_millis(unit));

        gw.y2_off();
        gw.g2_on();
        thread::sleep(Duration::from_millis(unit * 5));

        gw.g2_off();
        gw.y2_on();
        thread::sleep(Duration::from_millis(unit));

        gw.y2_off();
        gw.r2_on();
        thread::sleep(Duration::from_millis(unit));
    }
}
