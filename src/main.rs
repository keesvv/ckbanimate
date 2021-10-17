mod animation;
mod animations;
mod board;
mod control;
mod rgb;

use animation::Animation;
use control::Control;
use std::fs::OpenOptions;

use crate::animations::Marquee;

fn main() {
    let f_ctrl = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/input/ckb1/cmd") // TODO: pass through cmdline args
        .unwrap();

    let control = Control::new(f_ctrl);

    let mut marquee = Marquee::new(control);
    marquee.repeat();
}
