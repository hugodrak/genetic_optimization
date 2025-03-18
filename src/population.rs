//! population.rs
use crate::utils::{City, distance};
use rand::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Individual {
    pub route: Vec<usize>,     // A sequence of city indices
    pub fitness: f64,          // We can store fitness to avoid recomputing
}

impl Individual {
    /// Create a new Individual with a random route (permutation of city indices).
    /// The fitness will be computed as the negative route distance
    /// (so that "larger is better" in GA).
    pub fn new_random(cities: &Vec<City>) -> Self {
        let num_cities = cities.len();
        let mut route: Vec<usize> = (0..num_cities).collect();
        route.shuffle(&mut rand::thread_rng());

        let fit = compute_fitness(&route, cities);
        Individual {
            route,
            fitness: fit,
        }
    }
}

/// The population is simply a vector of Individuals.
#[derive(Debug)]
pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    /// Creates a new population of given size, each with a random route.
    pub fn new_random(pop_size: usize, cities: &Vec<City>) -> Self {
        let mut individuals = Vec::new();
        for _ in 0..pop_size {
            individuals.push(Individual::new_random(cities));
        }
        Population { individuals }
    }
}

/// The "fitness" can be negative of the route distance,
/// since the GA will try to maximize fitness.
pub fn compute_fitness(route: &[usize], cities: &Vec<City>) -> f64 {
    let mut total_distance = 0.0;
    for i in 0..route.len() - 1 {
        let city_a = &cities[route[i]];
        let city_b = &cities[route[i + 1]];
        total_distance += distance(city_a, city_b);
    }
    // If TSP is cyclical, add distance from last back to first
    let first_city = &cities[route[0]];
    let last_city  = &cities[route[route.len() - 1]];
    total_distance += distance(first_city, last_city);

    // Negate so smaller distance => higher fitness
    -total_distance
}
