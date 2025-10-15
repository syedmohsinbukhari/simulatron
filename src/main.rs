mod world;
mod world_objects;
mod simulatron;
mod visualization;

use clap::Parser;
use simulatron::Simulatron;

#[derive(Parser)]
#[command(name = "Simulatron")]
#[command(version = "1.0")]
#[command(about = "Simulates a 2D world", long_about = None)]
struct Cli {
    /// X dimension of the world
    x: u32,
    /// Y dimension of the world
    y: u32,
    /// Number of random world objects to initialize
    #[arg(short, long, default_value_t = 0)]
    num_objects: u32,
}

#[macroquad::main("Simulatron")]
async fn main() {
    let args = Cli::parse();
    let mut simulatron = Simulatron::new(args.x, args.y);
    simulatron.initialize_random_world_objects(args.num_objects);
    simulatron.visualize().await;
}
