//! Ternary percolation theory: cluster finding, threshold detection, conductance.

use std::collections::{HashSet, VecDeque};

/// Ternary grid for percolation
#[derive(Clone, Debug)]
pub struct TernaryGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<i8>,
}

impl TernaryGrid {
    pub fn new(width: usize, height: usize, fill: i8) -> Self {
        Self { width, height, cells: vec![fill; width * height] }
    }

    pub fn from_vec(width: usize, height: usize, cells: Vec<i8>) -> Self {
        assert_eq!(cells.len(), width * height);
        Self { width, height, cells }
    }

    pub fn get(&self, x: usize, y: usize) -> i8 { self.cells[y * self.width + x] }
    pub fn set(&mut self, x: usize, y: usize, v: i8) { self.cells[y * self.width + x] = v; }

    /// Site percolation: randomly activate sites with given probability
    pub fn site_percolation(&mut self, prob: f64, state: i8, rng: &[f64]) -> usize {
        let mut activated = 0;
        for i in 0..self.cells.len() {
            if rng[i % rng.len()] < prob && self.cells[i] == 0 {
                self.cells[i] = state;
                activated += 1;
            }
        }
        activated
    }

    /// Find connected components using flood fill
    pub fn find_components(&self, state: i8, connectivity: usize) -> Vec<Vec<(usize, usize)>> {
        let mut visited = vec![false; self.width * self.height];
        let mut components = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == state && !visited[y * self.width + x] {
                    let mut component = Vec::new();
                    let mut queue = VecDeque::new();
                    queue.push_back((x, y));
                    visited[y * self.width + x] = true;

                    while let Some((cx, cy)) = queue.pop_front() {
                        component.push((cx, cy));
                        let neighbors = self.neighbors(cx, cy, connectivity);
                        for (nx, ny) in neighbors {
                            let idx = ny * self.width + nx;
                            if !visited[idx] && self.get(nx, ny) == state {
                                visited[idx] = true;
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                    components.push(component);
                }
            }
        }
        components
    }

    fn neighbors(&self, x: usize, y: usize, connectivity: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if x > 0 { n.push((x-1, y)); }
        if x < self.width-1 { n.push((x+1, y)); }
        if y > 0 { n.push((x, y-1)); }
        if y < self.height-1 { n.push((x, y+1)); }
        if connectivity == 8 {
            if x > 0 && y > 0 { n.push((x-1, y-1)); }
            if x < self.width-1 && y > 0 { n.push((x+1, y-1)); }
            if x > 0 && y < self.height-1 { n.push((x-1, y+1)); }
            if x < self.width-1 && y < self.height-1 { n.push((x+1, y+1)); }
        }
        n
    }

    /// Check if any component of given state spans the grid (top to bottom)
    pub fn spans(&self, state: i8, connectivity: usize) -> bool {
        let components = self.find_components(state, connectivity);
        for comp in &components {
            let ys: HashSet<usize> = comp.iter().map(|&(_, y)| y).collect();
            if ys.contains(&0) && ys.contains(&(self.height - 1)) { return true; }
        }
        false
    }

    /// Check if ANY of the three states percolates
    pub fn any_percolates(&self, connectivity: usize) -> Option<i8> {
        for state in [-1i8, 0, 1] {
            if self.spans(state, connectivity) { return Some(state); }
        }
        None
    }

    /// Largest component size for a state
    pub fn largest_component(&self, state: i8, connectivity: usize) -> usize {
        self.find_components(state, connectivity)
            .into_iter()
            .map(|c| c.len())
            .max()
            .unwrap_or(0)
    }

    /// Sweep percolation threshold: find critical probability
    pub fn find_threshold(width: usize, height: usize, state: i8, rng: &[f64], steps: usize) -> f64 {
        let mut best_prob = 0.0;
        let mut threshold = 0.5;
        let n_cells = width * height;
        let rng_per_step = n_cells + 100;

        for step in 0..steps {
            let prob = (step as f64 + 0.5) / steps as f64;
            let mut grid = TernaryGrid::new(width, height, 0);
            let offset = step * rng_per_step;
            if offset + rng_per_step > rng.len() { continue; }
            grid.site_percolation(prob, state, &rng[offset..offset + n_cells]);
            if grid.spans(state, 4) {
                threshold = prob;
                break;
            }
        }
        threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rng(n: usize) -> Vec<f64> {
        (0..n).map(|i| {
            let s = (i as u64).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            s as f64 / u64::MAX as f64
        }).collect()
    }

    #[test]
    fn test_empty_grid_no_components() {
        let grid = TernaryGrid::new(5, 5, 0);
        let comps = grid.find_components(1, 4);
        assert!(comps.is_empty());
    }

    #[test]
    fn test_full_grid_single_component() {
        let grid = TernaryGrid::new(5, 5, 1);
        let comps = grid.find_components(1, 4);
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].len(), 25);
    }

    #[test]
    fn test_two_islands() {
        let mut grid = TernaryGrid::new(10, 10, 0);
        grid.set(1, 1, 1);
        grid.set(1, 2, 1);
        grid.set(8, 8, 1);
        let comps = grid.find_components(1, 4);
        assert_eq!(comps.len(), 2);
    }

    #[test]
    fn test_spanning_cluster() {
        let mut grid = TernaryGrid::new(5, 5, 0);
        for y in 0..5 { grid.set(2, y, 1); }
        assert!(grid.spans(1, 4));
    }

    #[test]
    fn test_no_span() {
        let mut grid = TernaryGrid::new(5, 5, 0);
        grid.set(2, 2, 1);
        assert!(!grid.spans(1, 4));
    }

    #[test]
    fn test_any_percolates() {
        let mut grid = TernaryGrid::new(5, 5, 0);
        for y in 0..5 { grid.set(0, y, -1); }
        assert_eq!(grid.any_percolates(4), Some(-1));
    }

    #[test]
    fn test_largest_component() {
        let mut grid = TernaryGrid::new(10, 10, 0);
        for y in 0..10 { for x in 0..3 { grid.set(x, y, 1); } }
        grid.set(8, 8, 1); // isolated
        let largest = grid.largest_component(1, 4);
        assert_eq!(largest, 30);
    }

    #[test]
    fn test_site_percolation() {
        let rng = make_rng(10000);
        let mut grid = TernaryGrid::new(20, 20, 0);
        let activated = grid.site_percolation(0.5, 1, &rng);
        assert!(activated > 0);
    }
}
