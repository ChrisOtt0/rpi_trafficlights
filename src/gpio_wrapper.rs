use std::process;

use rppal::gpio::{Gpio, OutputPin};

pub struct GpioWrapper {
    r1: OutputPin,
    y1: OutputPin,
    g1: OutputPin,

    r2: OutputPin,
    y2: OutputPin,
    g2: OutputPin
}

impl GpioWrapper {
    pub fn new(r1: u8, y1: u8, g1: u8, r2: u8, y2: u8, g2: u8) -> GpioWrapper {
        let gpio = match Gpio::new() {
            Ok(v) => v,
            _ => {
                println!("{}", create_error(String::from("GPIO"))); 
                process::exit(1);
            }
        };

        let r1 = get_pin(&gpio, r1);
        let y1 = get_pin(&gpio, y1);
        let g1 = get_pin(&gpio, g1);

        let r2 = get_pin(&gpio, r2);
        let y2 = get_pin(&gpio, y2);
        let g2 = get_pin(&gpio, g2);

        GpioWrapper { r1: r1, y1: y1, g1: g1, r2: r2, y2: y2, g2: g2 }
    }

    pub fn r1_on(&mut self) {
        self.r1.set_high();
    }

    pub fn y1_on(&mut self) {
        self.y1.set_high();
    }

    pub fn g1_on(&mut self) {
        self.g1.set_high();
    }

    pub fn r2_on(&mut self) {
        self.r2.set_high();
    }

    pub fn y2_on(&mut self) {
        self.y2.set_high();
    }

    pub fn g2_on(&mut self) {
        self.g2.set_high();
    }

    pub fn r1_off(&mut self) {
        self.r1.set_low();
    }

    pub fn y1_off(&mut self) {
        self.y1.set_low();
    }

    pub fn g1_off(&mut self) {
        self.g1.set_low();
    }

    pub fn r2_off(&mut self) {
        self.r2.set_low();
    }

    pub fn y2_off(&mut self) {
        self.y2.set_low();
    }

    pub fn g2_off(&mut self) {
        self.g2.set_low();
    }
}

fn get_pin(gpio: &Gpio, port: u8) -> OutputPin {
    match gpio.get(port) {
        Ok(v) => v.into_output(),
        _ => {
            println!("{}", create_error(String::from("OutputPin")));
            process::exit(1);
        }
    }
}


fn create_error(str: String) -> String {
    format!("Error creating {str} struct.")
}
