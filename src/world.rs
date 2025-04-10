pub struct World {
    x: u32,
    y: u32,
}

impl World {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

