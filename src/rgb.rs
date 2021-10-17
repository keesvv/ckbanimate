pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn get_hex(&self) -> String {
        return format!("{:02x}{:02x}{:02x}", self.0, self.1, self.2);
    }
}
