use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct Walker {
    id: usize,
    x: i32,
    y: i32,
}

impl Walker {
    fn new(id: usize) -> Walker {
        Walker { id, x: 0, y: 0 }
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..4);

        match direction {
            0 => self.x += 1,
            1 => self.x -= 1,
            2 => self.y += 1,
            3 => self.y -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

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
