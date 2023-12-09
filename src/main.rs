use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod models {
    pub mod walker;
}

use crate::models::walker::Walker;

fn simulate(walker: &Arc<Mutex<Walker>>, steps: usize) {
    for _ in 0..steps {
        let mut walker_guard = match walker.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("Error locking walker: {}", e);
                continue;
            }
        };
        walker_guard.step();

        println!(
            "Walker {} position: {:?}",
            walker_guard.id,
            walker_guard.position()
        );

        // Introduce a 1-second delay after each step
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    // Number of walkers
    let num_walkers = 3;
    // Number of steps each walker takes
    let steps_per_walker = 10;

    // Create a vector of Arc<Mutex<Walker>> for each walker
    let shared_walkers: Vec<Arc<Mutex<Walker>>> = (0..num_walkers)
        .map(|id| Arc::new(Mutex::new(Walker::new(id))))
        .collect();

    // Create threads for each walker
    let mut handles = vec![];

    for walker in &shared_walkers {
        let walker = Arc::clone(walker);
        let handle = thread::spawn(move || {
            // Simulate the walker
            simulate(&walker, steps_per_walker);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
