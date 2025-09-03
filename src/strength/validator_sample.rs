use crate::strength::stake_distribution::*;
use itertools::Itertools;

// Represents a single validator sample with some given underlying stake distribution.
pub struct ValidatorSample<'a, D: StakeDistribution> {
    distribution: &'a D,
    nodes: Vec<usize>
}

impl<'a, D: StakeDistribution> ValidatorSample<'a, D> {
    pub fn new(size: usize, distribution: &'a D) -> Self {
        let mut nodes = vec![0; size];
        for n in 0..size {
            nodes[n] = distribution.sample();
        }
        nodes.sort_unstable_by(|&a, &b| distribution.stake_of(a).total_cmp(&distribution.stake_of(b)));

        Self{
            distribution,
            nodes,
        }
    }

    pub fn total_stake(&self) -> f64 {
        self.nodes.iter().unique().map(|n| {self.distribution.stake_of(*n)}).sum()
    }

    pub fn strong_quorum_stake(&self) -> f64 {
        self.nodes.iter()// Iterate over the validator sample sorted by stake.
            .take(self.nodes.len() - (self.nodes.len() / 3))// Take the "poorest" two thirds
            .unique()// Count each validator only once
            .map(|n| {self.distribution.stake_of(*n)}) // Map validators to their stakes.
            .sum() // Sum the stakes of all validators in the "poorest" 2/3 of the seats.
    }

    pub fn weak_quorum_stake(&self) -> f64 {
        self.nodes.iter()// Iterate over the validator sample sorted by stake.
            .take((self.nodes.len() + 2) / 3)// Take the "poorest" two thirds
            .unique()// Count each validator only once
            .map(|n| {self.distribution.stake_of(*n)}) // Map validators to their stakes.
            .sum() // Sum the stakes of all validators in the "poorest" 2/3 of the seats.
    }
}
