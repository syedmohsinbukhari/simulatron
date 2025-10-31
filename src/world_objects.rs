pub struct WorldObject {
    pub x: u32,
    pub y: u32,
    pub vx: i32,
    pub vy: i32,
}

impl WorldObject {
    pub fn new(x: u32, y: u32, vx: i32, vy: i32) -> Self {
        Self { x, y, vx, vy }
    }

    pub fn move_object(&mut self, max_x: u32, max_y: u32) {
        let mut new_x = self.x as i32 + self.vx;
        let mut new_y = self.y as i32 + self.vy;

        if new_x < 0 {
            new_x = 0;
            self.vx *= -1;
        } else if new_x >= max_x as i32 {
            new_x = max_x as i32 - 1;
            self.vx *= -1;
        }

        if new_y < 0 {
            new_y = 0;
            self.vy *= -1;
        } else if new_y >= max_y as i32 {
            new_y = max_y as i32 - 1;
            self.vy *= -1;
        }

        self.x = new_x as u32;
        self.y = new_y as u32;
    }
}
