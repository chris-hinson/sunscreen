#[derive(Clone)]
pub struct Vram {
    pub contents: [u8; 2048],
}

impl Vram {
    pub fn new() -> Self {
        Vram {
            contents: [0; 2048],
        }
    }
}
