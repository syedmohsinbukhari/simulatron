use macroquad::prelude::*;
use crate::world::World;


fn draw_grid(screen_width: f32, screen_height: f32, world_x: u32, world_y: u32, cell_width: f32, cell_height: f32) {
    let dash_length = 5.0;
    let gap_length = 5.0;

    for i in 0..=world_x {
        let x_coord = i as f32 * cell_width;
        for y_segment_start in (0..screen_height as u32).step_by((dash_length + gap_length) as usize) {
            let y_start = y_segment_start as f32;
            let y_end = (y_start + dash_length).min(screen_height);
            draw_line(x_coord, y_start, x_coord, y_end, 1.0, GRAY);
        }
    }
    for i in 0..=world_y {
        let y_coord = i as f32 * cell_height;
        for x_segment_start in (0..screen_width as u32).step_by((dash_length + gap_length) as usize) {
            let x_start = x_segment_start as f32;
            let x_end = (x_start + dash_length).min(screen_width);
            draw_line(x_start, y_coord, x_end, y_coord, 1.0, GRAY);
        }
    }
}

pub async fn visualize_world(world: &World) {
    let (screen_width, screen_height) = (800.0, 600.0);
    let (world_x, world_y) = world.dimensions();

    request_new_screen_size(screen_width, screen_height + 30.0);

    loop {
        clear_background(LIGHTGRAY);

        let cell_width = screen_width / world_x as f32;
        let cell_height = screen_height / world_y as f32;

        // Draw grid
        draw_grid(screen_width, screen_height, world_x, world_y, cell_width, cell_height);

        // Draw world objects
        for obj in world.world_objects() {
            let obj_x = obj.x as f32 * cell_width + cell_width / 2.0;
            let obj_y = obj.y as f32 * cell_height + cell_height / 2.0;
            draw_circle(obj_x, obj_y, cell_width / 3.0, RED);
        }

        next_frame().await
    }
}
