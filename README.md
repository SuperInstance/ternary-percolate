# ternary-percolate

Percolation theory on ternary grids — where every site is blocked, open, or conducting, and the question is always: does a path exist?

## Why This Exists

Classical percolation is binary: sites are open or closed. But real systems have three relevant states. In material science: insulator, semiconductor, conductor. In epidemiology: immune, susceptible, infected. In network routing: down, degraded, up. Binary percolation can't model the middle state — the one that *might* conduct, *might* transmit, *might* route.

Ternary percolation adds a third state `{−1, 0, +1}` and asks: for which states does a spanning cluster exist? The answer is non-trivial. You can have a grid where state `+1` percolates (top to bottom), state `−1` percolates independently, and state `0` is fragmented. Or all three fail. Or only one succeeds. The percolation threshold depends on which state you're measuring.

This crate implements flood-fill cluster finding, spanning detection, threshold estimation, and component analysis — all generalized to ternary grids.

## Architecture

```
TernaryGrid (width × height, cells: Vec<i8>)
    │
    ├── get(x, y) / set(x, y, v) ──► cell access
    ├── site_percolation(prob, state, rng) ──► randomly activate sites
    ├── find_components(state, connectivity) ──► flood fill BFS
    │       └── neighbors(x, y, 4|8) ──► 4-connectivity or 8-connectivity
    ├── spans(state, connectivity) ──► does any component touch top AND bottom?
    ├── any_percolates(connectivity) ──► check all three states
    ├── largest_component(state, connectivity) ──► size of biggest cluster
    └── find_threshold(w, h, state, rng, steps) ──► sweep to find critical probability
```

**Key types:**

- **`TernaryGrid`** — a `width × height` grid where each cell holds an `i8` in `{−1, 0, +1}`. All percolation operations are methods on this struct.
- Connectivity is either **4-connected** (cardinal neighbors) or **8-connected** (includes diagonals). This affects cluster geometry and percolation thresholds.

## Usage

```rust
use ternary_percolate::TernaryGrid;

// Create a 10×10 grid, initially all zeros
let mut grid = TernaryGrid::new(10, 10, 0);

// Place some conducting (+1) sites
grid.set(2, 0, 1);
grid.set(2, 1, 1);
grid.set(2, 2, 1);
grid.set(2, 3, 1);
grid.set(2, 4, 1);
// ... continuing to row 9 creates a spanning column

// Check if state +1 percolates (top to bottom)
assert!(grid.spans(1, 4)); // 4-connected

// Find connected components
let components = grid.find_components(1, 4);
println!("Found {} components of state +1", components.len());

// Largest component size
let largest = grid.largest_component(1, 4);

// Check if ANY of the three states percolates
grid.set(0, 0, -1); // place some -1 sites
match grid.any_percolates(4) {
    Some(state) => println!("State {} percolates!", state),
    None => println!("No state percolates"),
}

// Random site percolation
let mut rng = vec![0.3, 0.7, 0.1, 0.9, /* ... enough values ... */];
let mut grid = TernaryGrid::new(20, 20, 0);
let activated = grid.site_percolation(0.5, 1, &rng);
// 50% probability of activating each site to state +1

// Threshold estimation: sweep probabilities to find critical point
let threshold = TernaryGrid::find_threshold(20, 20, 1, &rng, 100);
// Returns the probability where spanning first occurs
```

## API Reference

### `TernaryGrid`

| Method | Description |
|--------|-------------|
| `TernaryGrid::new(width, height, fill)` | Create grid filled with `fill` ∈ `{−1, 0, +1}` |
| `TernaryGrid::from_vec(width, height, cells)` | Create from existing cell data |
| `.get(x, y)` | Get cell value |
| `.set(x, y, v)` | Set cell value |
| `.site_percolation(prob, state, rng)` | Randomly activate `0`-valued cells to `state` with given probability. Returns count activated. |
| `.find_components(state, connectivity)` | Flood-fill BFS to find all connected components of `state`. Connectivity: 4 or 8. Returns `Vec<Vec<(usize, usize)>>`. |
| `.spans(state, connectivity)` | True if any component of `state` contains cells in both row 0 and row `height-1`. |
| `.any_percolates(connectivity)` | Check all three states `{-1, 0, +1}`. Returns `Some(state)` for the first that spans. |
| `.largest_component(state, connectivity)` | Size of the largest connected component for `state`. |
| `TernaryGrid::find_threshold(w, h, state, rng, steps)` | Sweep percolation probabilities from 0 to 1, return the threshold where spanning first occurs. |

## The Deeper Idea

Percolation theory studies the emergence of long-range connectivity in random systems. The critical insight is the **phase transition**: below a critical probability `p_c`, only small isolated clusters exist. Above `p_c`, a giant component spans the system. This transition is sharp — it's a genuine mathematical discontinuity in the infinite-system limit.

Ternary percolation compounds this: you have *three* competing phase transitions, one per state. As you fill a grid with `+1` sites, you're simultaneously reducing the space available for `−1` and `0`. The percolation thresholds are coupled: if `+1` occupies more than `p_c` of the grid, neither `−1` nor `0` can percolate (they don't have enough sites). This creates a three-way competition that doesn't exist in binary percolation.

The practical implications span materials science (composite conductivity), epidemiology (multi-strain spread models), and network reliability (survive-qualify-thrive states). The ternary framework captures a qualitative distinction that binary models miss: the *middle state* — neither blocked nor conducting — that can participate in clusters without guaranteeing connectivity.

## Related Crates

- **`ternary-morphogenesis`** — reaction-diffusion on ternary grids, where percolation determines pattern connectivity
- **`ternary-renormalization`** — coarse-graining of ternary fields, which transforms percolation thresholds across scales
- **`ternary-route`** — routing through ternary networks, where percolation determines which routes are viable
