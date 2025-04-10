use crate::world::World;

pub struct Simulatron {
    world: World,
}

impl Simulatron {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            world: World::new(x, y),
        }
    }

    pub fn display_world(&self) {
        let (x, y) = self.world.dimensions();
        println!("Simulatron created with world dimensions: {} x {}", x, y);
    }
}

