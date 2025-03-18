Below is a **Rust** project demonstrating a **Genetic Algorithm (GA)** for solving a simplified **Traveling Salesman Problem (TSP)**. The code is split into multiple files (`main.rs`, `genetic.rs`, `population.rs`, and `utils.rs`) plus a sample data file (`cities.txt`). This structure illustrates how to organize a Rust project for GA-based optimization.

You can extend or optimize this code further (e.g., more sophisticated selection, crossover, mutation strategies) for real-world performance.

---

## File Structure

```
/
├── Cargo.toml
├── data/
│   └── cities.txt
└── src/
    ├── main.rs
    ├── genetic.rs
    ├── population.rs
    └── utils.rs
```

### About Each File

1. **`Cargo.toml`** – Defines the Rust package and any dependencies.  
2. **`data/cities.txt`** – A simple data file listing city coordinates.  
3. **`main.rs`** – Entry point; parses data, runs the GA, and prints the result.  
4. **`genetic.rs`** – Houses the genetic algorithm logic (selection, crossover, mutation).  
5. **`population.rs`** – Defines `Individual` and `Population`, how to compute fitness, etc.  
6. **`utils.rs`** – Utility functions (e.g., parsing city data, computing distances).

---

# Building & Running

1. **Install** [Rust](https://www.rust-lang.org/tools/install).  
2. **Clone** or create the directory structure described above.  
3. **Place city data** in `data/cities.txt`.  
4. **Build** and **run**:
   ```bash
   cd Genetic-Optimization
   cargo run
   ```

You should see output like:
```
Loaded 10 cities from data/cities.txt
Running GA with pop_size=50, generations=100
Best route found: [ ...some permutation... ]
Distance: 123.45
```

---

# Possible Enhancements

- **Elaborate on crossover**: You could implement **PMX** (Partially Mapped Crossover) or other TSP-specific crossovers.  
- **Multi-threading**: Use [Rayon](https://docs.rs/rayon/latest/rayon/) or similar to parallelize fitness evaluations.  
- **Elitism**: More advanced elitism strategies can preserve multiple top individuals.  
- **Adaptive mutation/crossover rates**: Tweak probabilities during evolution.  
- **Hybridization**: Combine GA with local search (like 2-opt or 3-opt) for stronger solutions.

---

## Summary

This Rust code demonstrates a straightforward **Genetic Algorithm** for TSP-like route optimization. It:

1. **Parses** city coordinates from a file.  
2. **Initializes** a random population of permutations (routes).  
3. **Evolves** the population via selection, crossover, mutation, and elitism.  
4. **Prints** out the best solution found after a given number of generations.