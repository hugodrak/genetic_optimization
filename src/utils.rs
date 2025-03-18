//! utils.rs
//!
//! Utility functions for TSP: city definition, distance calculations,
//! and parsing city data from a file.

use std::fs::File;
use std::io::{BufRead, BufReader};

/// A basic city representation with an ID and coordinates.
#[derive(Debug, Clone)]
pub struct City {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

/// Reads city data from a file (e.g., "data/cities.txt").
pub fn parse_cities(filename: &str) -> Vec<City> {
    let file = File::open(filename).unwrap_or_else(|_| {
        panic!("Failed to open file: {}", filename);
    });
    let reader = BufReader::new(file);

    let mut cities = Vec::new();

    for line in reader.lines() {
        if let Ok(entry) = line {
            let parts: Vec<&str> = entry.split_whitespace().collect();
            if parts.len() >= 3 {
                let id = parts[0].parse::<usize>().unwrap_or(0);
                let x  = parts[1].parse::<f64>().unwrap_or(0.0);
                let y  = parts[2].parse::<f64>().unwrap_or(0.0);

                cities.push(City { id, x, y });
            }
        }
    }
    cities
}

/// Computes the Euclidean distance between two cities.
pub fn distance(a: &City, b: &City) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}
