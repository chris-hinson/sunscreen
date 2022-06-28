#[derive(Clone)]
pub struct Wram {
    pub contents: [u8; 2048],
}

impl Wram {
    pub fn new() -> Self {
        Wram {
            contents: [0; 2048],
        }
    }
}
