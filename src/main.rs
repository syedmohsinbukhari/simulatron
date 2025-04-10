mod world;
mod simulatron;

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
}

fn main() {
    let args = Cli::parse();
    let simulatron = Simulatron::new(args.x, args.y);
    simulatron.display_world();
}
