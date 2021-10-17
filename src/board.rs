pub struct Loc(pub usize, pub usize);

impl Loc {
    pub fn get_key(&self) -> &'static str {
        return get_keymap()[self.1][self.0];
    }
}

pub fn get_keymap() -> Vec<Vec<&'static str>> {
    return vec![
        vec![
            "esc", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
        ],
        vec![
            "grave", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "minus", "equal", "bspace",
        ],
        vec![
            "tab", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "lbrace", "rbrace", "bslash",
        ],
        vec![
            "caps", "a", "s", "d", "f", "g", "h", "j", "k", "l", "colon", "quote", "enter",
        ],
        // TODO: finish the keymap
    ];
}
