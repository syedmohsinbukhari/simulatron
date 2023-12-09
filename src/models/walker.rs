use rand::Rng;

#[derive(Debug)]
pub struct Walker {
    pub id: usize,
    pub x: i32,
    pub y: i32,
}

impl Walker {
    pub fn new(id: usize) -> Walker {
        Walker { id, x: 0, y: 0 }
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..4);

        match direction {
            0 => self.x += 1,
            1 => self.x -= 1,
            2 => self.y += 1,
            3 => self.y -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
