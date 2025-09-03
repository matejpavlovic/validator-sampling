use statrs::statistics::Statistics;
use crate::strength::stake_distribution::StakeDistribution;
use crate::strength::validator_sample::ValidatorSample;

// Simulation represents multiple iterations of validator sampling.
// It is only concerned with the strength of the sample
// (i.e., the amount of stake the validators have), not the failure probability
// (i.e., it does not distinguish between correct and faulty nodes). Failure probabilities
// are handled by the representativeness crate.
pub struct Simulation<'a, D: StakeDistribution> {
    distribution: &'a D,

    weak_quorum_stakes: Vec<f64>,
    strong_quorum_stakes: Vec<f64>,
    total_stakes: Vec<f64>,
}

impl<'a, D: StakeDistribution> Simulation<'a, D> {
    pub fn new(distribution: &'a D) -> Self {
        Self {
            total_stakes: vec![],
            strong_quorum_stakes: vec![],
            weak_quorum_stakes: vec![],
            distribution
        }
    }

    pub fn run(&mut self, sample_size: usize, iterations: usize) {
        for _ in 0..iterations {
            let sample = ValidatorSample::new(sample_size, self.distribution);
            self.weak_quorum_stakes.push(sample.weak_quorum_stake());
            self.strong_quorum_stakes.push(sample.strong_quorum_stake());
            self.total_stakes.push(sample.total_stake());
        }
    }

    // Returns the average and standard deviation of the amount of stake backing a strong quorum
    // of a validator sample. A strong quorum is the set of 2/3 of the validators
    // (i.e., a set sufficient to endorse a state transition). Out of all the possible subsets
    // of a sample, we consider the one with the smallest stake.
    // The amount of stake is represented as a fraction of the total stake present in the system.
    pub fn strong_quorum_stake(&self) -> (f64, f64) {
        (
            self.strong_quorum_stakes.iter().mean(),
            self.strong_quorum_stakes.iter().std_dev(),
        )
    }

    // Same as strong_quorum_stake(), except the quorum size is 1/3 instead of 2/3.
    pub fn weak_quorum_stake(&self) -> (f64, f64) {
        (
            self.weak_quorum_stakes.iter().mean(),
            self.weak_quorum_stakes.iter().std_dev(),
        )
    }

    // Returns the combined stake of all the nodes in the sample.
    // The amount of stake is represented as a fraction of the total stake present in the system.
    pub fn total_stakes(&self) -> (f64, f64) {
        (
            self.total_stakes.iter().mean(),
            self.total_stakes.iter().std_dev(),
        )
    }
}