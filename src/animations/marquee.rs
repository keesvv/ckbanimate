use std::{thread::sleep, time::Duration};

use crate::{
    animation::Animation,
    board::{get_keymap, Loc},
    control::Control,
    rgb::RGB,
};

pub struct Marquee {
    control: Control,
}

impl Marquee {
    pub fn new(control: Control) -> Self {
        Self { control }
    }
}

impl Animation for Marquee {
    fn start(&mut self) {
        let keymap = get_keymap();

        for y in 0..keymap.len() {
            for x in 0..keymap[y].len() {
                self.control.write_rgb(vec![Loc(x, y)], RGB(255, 255, 255));

                // Sleep 100 ms before going to the next key
                sleep(Duration::from_millis(100));

                self.control.write_rgb(vec![Loc(x, y)], RGB(0, 0, 0));
            }
        }
    }
}
