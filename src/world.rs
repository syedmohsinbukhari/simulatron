use crate::world_objects::WorldObject;

pub struct World {
    x: u32,
    y: u32,
    world_objects: Vec<WorldObject>,
}

impl World {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y, world_objects: Vec::new() }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn add_world_object(&mut self, obj: WorldObject) {
        self.world_objects.push(obj);
    }

    pub fn update(&mut self) {
        let (max_x, max_y) = self.dimensions();
        for obj in self.world_objects.iter_mut() {
            obj.move_object(max_x, max_y);
        }
    }

    pub fn world_objects(&self) -> &Vec<WorldObject> {
        &self.world_objects
    }

    pub fn destroy_all_objects(&mut self) {
        self.world_objects.clear();
    }
}

