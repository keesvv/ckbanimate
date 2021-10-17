use crate::{
    board::{get_keymap, Loc},
    rgb::RGB,
};
use std::{fs::File, io::Write};

pub struct Control {
    stream: File,
}

impl Control {
    pub fn new(stream: File) -> Self {
        Self { stream }
    }

    pub fn write_rgb(&mut self, locs: Vec<Loc>, color: RGB) {
        let mapping: Vec<String> = locs
            .into_iter()
            .map(|loc| format!("{}:{}", loc.get_key(), color.get_hex()))
            .collect();

        println!("writing mapping: {:?}", mapping);

        self.stream
            .write(format!("rgb {}\n", mapping.join(" ")).as_bytes())
            .unwrap();
    }

    pub fn clear_board(&mut self) {
        let keymap = get_keymap();

        // TODO: map everything into one command
        for y in 0..keymap.len() {
            for x in 0..keymap[y].len() {
                self.write_rgb(vec![Loc(x, y)], RGB(0, 0, 0))
            }
        }
    }
}
