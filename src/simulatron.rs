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
        if !self.world.world_objects().is_empty() {
            println!("World objects already exist. Destroy them first to create new ones.");
            return;
        }
        if n == 0 {
            println!("No objects requested to be created.");
            return;
        }
        let (max_x, max_y) = self.world.dimensions();
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let x = rng.gen_range(0..max_x);
            let y = rng.gen_range(0..max_y);

            let velocities = [(-1, 0), (0, -1), (1, 0), (0, 1)];
            let (vx, vy) = velocities[rng.gen_range(0..velocities.len())];

            let obj = WorldObject::new(x, y, vx, vy);
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

    pub fn step(&mut self) {
        self.world.update();
    }

    pub fn destroy_objects(&mut self) {
        self.world.destroy_all_objects();
    }

    pub async fn visualize(&mut self) {
        self.display_world();
        visualization::visualize_world(&self.world).await
    }
}

