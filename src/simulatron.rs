use crate::world::World;
use crate::world_objects::WorldObject;
use rand::Rng;

use crate::visualization;

pub struct Simulatron {
    world: World,
}

impl Simulatron {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            world: World::new(x, y),
        }
    }

    pub fn initialize_random_world_objects(&mut self, n: u32) {
        let (max_x, max_y) = self.world.dimensions();
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let x = rng.gen_range(0..max_x);
            let y = rng.gen_range(0..max_y);
            let obj = WorldObject::new(x, y);
            self.world.add_world_object(obj);
        }
    }

    pub fn display_world(&self) {
        let (x, y) = self.world.dimensions();
        println!("Simulatron created with world dimensions: {} x {}", x, y);
        println!("World objects:");
        for obj in self.world.world_objects() {
            println!("  Object at: ({}, {})", obj.x, obj.y);
        }
    }

    pub async fn visualize(&self) {
        self.display_world();
        visualization::visualize_world(&self.world).await
    }
}

