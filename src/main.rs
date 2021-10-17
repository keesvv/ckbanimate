mod animation;
mod board;
mod control;
mod rgb;

use control::Control;
use rgb::RGB;
use std::{fs::OpenOptions, thread::sleep, time::Duration};

use crate::board::Loc;

fn main() {
    let f_ctrl = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/input/ckb1/cmd") // TODO: pass through cmdline args
        .unwrap();

    let mut control = Control::new(f_ctrl);

    // TODO: remove
    control.write_rgb(vec![Loc(0, 0), Loc(1, 0)], RGB(255, 0, 0));

    sleep(Duration::from_secs(2));

    control.clear_board();
}
