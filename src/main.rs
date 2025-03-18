//! main.rs
use genetic_optimization::genetic::run_ga; // If you're using a workspace/crate structure
// Otherwise, if everything is in the same project, do:
mod genetic;
mod population;
mod utils;

use crate::genetic::run_ga;
use crate::utils::parse_cities;

fn main() {
    // 1. Load city data
    let filename = "data/cities.txt";
    let cities = parse_cities(filename);
    if cities.is_empty() {
        eprintln!("No cities found in {}. Exiting.", filename);
        return;
    }
    println!("Loaded {} cities from {}", cities.len(), filename);

    // 2. GA hyperparameters
    let population_size = 50;
    let generations = 100;
    let crossover_rate = 0.8;
    let mutation_rate = 0.02;
    let tournament_size = 3;

    // 3. Run the GA
    println!("Running GA with pop_size={}, generations={}", population_size, generations);
    let best = run_ga(
        &cities,
        population_size,
        generations,
        crossover_rate,
        mutation_rate,
        tournament_size,
    );

    // 4. Print best route and its distance
    // Remember, "fitness" = -distance
    let best_distance = -best.fitness;
    println!("Best route found: {:?}", best.route);
    println!("Distance: {:.2}", best_distance);
}
