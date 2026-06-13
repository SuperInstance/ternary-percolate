# ternary-percolate

Percolation theory on ternary 2D grids. Cluster finding via flood-fill BFS, spanning detection, percolation threshold estimation, and multi-state connectivity for {-1, 0, +1} lattices.

## Why It Matters

Percolation theory studies how connections form across random media. In ternary systems, three distinct "phases" compete for spatial dominance — each can form its own spanning cluster. The critical question: at what density does a phase transition occur, creating a path from one side of the system to the other?

This matters for:
- **Ternary network connectivity**: when does a community of like-minded agents span the network?
- **Opinion dynamics**: at what density does a ternary opinion become globally dominant?
- **Ternary image segmentation**: which regions are connected?
- **Phase transitions**: studying the universality class of ternary percolation vs. classical binary

## How It Works

### Site Percolation Model

On a grid of size $W \times H$, each cell holds a ternary value $v \in \{-1, 0, +1\}$. The percolation probability $p$ determines how many cells are "activated" (non-zero).

### Connected Components (Flood Fill)

Find connected components using BFS with 4- or 8-connectivity. For each cell with value $v$, explore all connected neighbors of the same value.

**Complexity:** O($W \times H$) time and space — each cell visited exactly once.

### Spanning Cluster Detection

A component **spans** the grid if it touches both the top row ($y = 0$) and bottom row ($y = H-1$):

$$\exists \text{ component } C: \{(x, 0) \in C\} \wedge \{(x, H-1) \in C\}$$

### Multi-State Percolation

Unlike binary percolation, ternary grids allow three independent percolation problems — one per state. `any_percolates()` checks all three and returns the first state that spans.

### Threshold Estimation

Sweep over probability $p \in (0, 1)$ in discrete steps. For each $p$:
1. Fill the grid randomly with density $p$
2. Check for spanning cluster
3. The threshold $p_c$ is the lowest $p$ where spanning occurs

**Classical threshold (square lattice, site percolation):** $p_c \approx 0.5928$ for binary.

**Complexity:** O($W \times H \times \text{steps}$) for a full sweep.

### Percolation Strength

$$P_\infty = \frac{\text{size of largest cluster}}{W \times H}$$

Below $p_c$: $P_\infty \sim O(\log N)$. Above $p_c$: $P_\infty$ grows as a power law $(p - p_c)^\beta$ where $\beta = 5/36$ in 2D.

## Quick Start

```rust
use ternary_percolate::*;

// Create a grid
let mut grid = TernaryGrid::new(10, 10, 0);

// Manually create a spanning column
for y in 0..10 { grid.set(5, y, 1); }
assert!(grid.spans(1, 4));  // +1 spans top to bottom

// Two disconnected islands
let mut grid2 = TernaryGrid::new(10, 10, 0);
grid2.set(1, 1, 1);
grid2.set(1, 2, 1);
grid2.set(8, 8, 1);
let comps = grid2.find_components(1, 4);
assert_eq!(comps.len(), 2);

// Check which states percolate
let mut grid3 = TernaryGrid::new(5, 5, 0);
for y in 0..5 { grid3.set(0, y, -1); }
assert_eq!(grid3.any_percolates(4), Some(-1));

// Largest component
let mut grid4 = TernaryGrid::new(10, 10, 0);
for y in 0..10 { for x in 0..3 { grid4.set(x, y, 1); } }
grid4.set(8, 8, 1); // isolated
assert_eq!(grid4.largest_component(1, 4), 30);
```

## API

| Method | Description |
|---|---|
| `TernaryGrid::new(w, h, fill)` | Create grid |
| `.get(x, y) / .set(x, y, v)` | Cell access |
| `.site_percolation(prob, state, rng)` | Randomly activate sites |
| `.find_components(state, conn) → Vec<Vec<(usize,usize)>>` | All connected clusters of given state |
| `.spans(state, conn) → bool` | Does any cluster touch top and bottom? |
| `.any_percolates(conn) → Option<i8>` | First state (of -1, 0, +1) that spans |
| `.largest_component(state, conn) → usize` | Size of biggest cluster |
| `TernaryGrid::find_threshold(w, h, state, rng, steps)` | Estimate critical probability |

## Architecture Notes

Percolation on ternary grids extends the **γ + η = C** conservation framework into the spatial domain. The +1 regions (constructive mass γ) and -1 regions (inhibitory mass η) each attempt to form spanning clusters, while the 0 regions (neutral substrate) act as barriers. The conserved quantity $C = (W \times H)$ constrains the total area — as γ grows, either η or the neutral background must shrink.

The percolation threshold for ternary systems is shifted relative to binary percolation because three phases compete for the same lattice sites. Near $p_c$, the system exhibits critical phenomena: cluster size distributions follow power laws $n(s) \sim s^{-\tau}$ with $\tau = 187/91$ (2D universality class), and correlation lengths diverge as $\xi \sim |p - p_c|^{-\nu}$ with $\nu = 4/3$.

## References

- Stauffer, D. & Aharony, A. (2018). *Introduction to Percolation Theory.* 2nd ed. CRC Press.
- Grimmett, G. (1999). *Percolation.* 2nd ed. Springer.
- Broadbent, S. R. & Hammersley, J. M. (1957). *Percolation Processes I.* Proc. Cambridge Phil. Soc.
- Sahimi, M. (1994). *Applications of Percolation Theory.* Taylor & Francis.

## License

MIT
