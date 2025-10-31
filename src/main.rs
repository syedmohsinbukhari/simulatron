mod world;
mod world_objects;
mod simulatron;
mod visualization;
// mod operations_queue;

use clap::Parser;
use simulatron::Simulatron;
use std::io;
use std::io::Write;

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

    loop {
        print!("what to do: ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut user_input_buffer = String::new();
        io::stdin().read_line(&mut user_input_buffer).expect("Failed to read line");
        let user_input = user_input_buffer.trim();

        let parts: Vec<&str> = user_input.splitn(2, ' ').collect();
        let command = parts[0];
        let mut amount: u32 = 1;

        if parts.len() > 1 {
            if let Ok(parsed_amount) = parts[1].parse::<u32>() {
                amount = parsed_amount;
            } else {
                println!("Invalid amount specified. Defaulting to 1.");
            }
        }

        match command {
            "step" => {
                for _ in 0..amount {
                    simulatron.step();
                    simulatron.visualize().await;
                }
            },
            "create" => {
                simulatron.initialize_random_world_objects(amount);
                simulatron.visualize().await;
            },
            "destroy" => {
                simulatron.destroy_objects();
                simulatron.visualize().await;
            },
            "exit" => {
                println!("Exiting Simulatron.");
                break;
            },
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
