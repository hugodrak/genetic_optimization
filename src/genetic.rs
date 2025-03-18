//! genetic.rs
use crate::population::{Population, Individual, compute_fitness};
use crate::utils::City;
use rand::prelude::*;
use std::cmp::Ordering;

/// Run the genetic algorithm for a number of generations.
pub fn run_ga(
    cities: &Vec<City>,
    population_size: usize,
    generations: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    tournament_size: usize,
) -> Individual {
    // 1. Create initial population
    let mut population = Population::new_random(population_size, cities);

    // 2. Evolution loop
    for _ in 0..generations {
        population = evolve_population(&population, cities, crossover_rate, mutation_rate, tournament_size);
    }

    // Return best individual
    find_best_individual(&population)
}

/// Produces the next generation from the current population.
fn evolve_population(
    current_pop: &Population,
    cities: &Vec<City>,
    crossover_rate: f64,
    mutation_rate: f64,
    tournament_size: usize,
) -> Population {
    let mut new_population = Vec::new();
    let pop_size = current_pop.individuals.len();

    // Keep the best individual (elitism)
    let best_ind = find_best_individual(current_pop);
    new_population.push(best_ind.clone());

    // Fill the rest of population
    while new_population.len() < pop_size {
        // Selection
        let parent1 = tournament_selection(current_pop, tournament_size);
        let parent2 = tournament_selection(current_pop, tournament_size);

        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        // Crossover
        let prob: f64 = rand::random();
        if prob < crossover_rate {
            order_crossover(&mut child1, &mut child2);
            // Recompute fitness
            child1.fitness = compute_fitness(&child1.route, cities);
            child2.fitness = compute_fitness(&child2.route, cities);
        }

        // Mutation
        mutate(&mut child1, mutation_rate, cities);
        mutate(&mut child2, mutation_rate, cities);

        // Add to new population
        new_population.push(child1);
        if new_population.len() < pop_size {
            new_population.push(child2);
        }
    }

    Population { individuals: new_population }
}

/// Returns the best individual (highest fitness) in the population.
fn find_best_individual(pop: &Population) -> Individual {
    pop.individuals
        .iter()
        .cloned()
        .max_by(|a, b| {
            a.fitness
                .partial_cmp(&b.fitness)
                .unwrap_or(Ordering::Equal)
        })
        .unwrap()
}

/// Selects an individual using tournament selection.
fn tournament_selection(pop: &Population, tournament_size: usize) -> Individual {
    let mut rng = rand::thread_rng();
    let pop_size = pop.individuals.len();

    let mut best = None;
    for _ in 0..tournament_size {
        let idx = rng.gen_range(0..pop_size);
        let candidate = pop.individuals[idx].clone();
        if best.is_none() || candidate.fitness > best.as_ref().unwrap().fitness {
            best = Some(candidate);
        }
    }
    best.unwrap()
}

/// Order Crossover (OX) for TSP permutations.
/// This function modifies both child1 and child2 in place.
fn order_crossover(child1: &mut Individual, child2: &mut Individual) {
    let size = child1.route.len();
    let mut rng = rand::thread_rng();

    // choose random segment
    let start = rng.gen_range(0..size);
    let end = rng.gen_range(start..size);

    // Copy segment from child1 to temp
    let slice1 = &child1.route[start..end];
    let slice2 = &child2.route[start..end];

    // We'll store which cities are used in that slice
    let mut used1 = vec![false; size];
    let mut used2 = vec![false; size];
    for &gene in slice1 {
        used1[gene] = true;
    }
    for &gene in slice2 {
        used2[gene] = true;
    }

    // Fill child1 from child2 outside the segment
    let mut idx = end;
    for &gene in child2.route.iter().cycle() {
        if !used1[gene] {
            child1.route[idx % size] = gene;
            idx += 1;
            if idx % size == start {
                break;
            }
        }
    }

    // Fill child2 from child1 outside the segment
    idx = end;
    for &gene in child1.route.iter().cycle() {
        if !used2[gene] {
            child2.route[idx % size] = gene;
            idx += 1;
            if idx % size == start {
                break;
            }
        }
    }
}

/// Swap mutation: with a small probability, randomly swap two cities in the route.
fn mutate(individual: &mut Individual, mutation_rate: f64, cities: &Vec<City>) {
    let mut rng = rand::thread_rng();
    let prob: f64 = rng.gen();

    if prob < mutation_rate {
        let size = individual.route.len();
        let i = rng.gen_range(0..size);
        let j = rng.gen_range(0..size);
        individual.route.swap(i, j);

        // Recompute fitness after mutation
        individual.fitness = compute_fitness(&individual.route, cities);
    }
}
