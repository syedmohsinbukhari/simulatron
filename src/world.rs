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

    pub fn world_objects(&self) -> &Vec<WorldObject> {
        &self.world_objects
    }
}

