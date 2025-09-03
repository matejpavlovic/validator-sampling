use rand::Rng;
use std::{fs};
use std::str::FromStr;
use num_bigint::BigInt;
use num_rational::BigRational;
use rand_distr::num_traits::ToPrimitive;

// StakeDistribution represents the assignment of stake to nodes.
// The stake value is a number between 0 and 1.
// The total stake of all nodes sums up to 1.
// Node IDs are integers starting at 0.
pub trait StakeDistribution {
    // Returns the amount of stake associated with a given node.
    // Returns 0.0 if node >= num_nodes.
    fn stake_of(&self, node: usize) -> f64;
    fn num_nodes(&self) -> usize;

    // Returns a random node id, where the probability of picking a particular node is
    // proportional to the node's stake.
    fn sample(&self) -> usize {
        let random_stake : f64 = rand::rng().random();
        let mut sum = 0.0;
        for i in 0..self.num_nodes() {
            sum += self.stake_of(i);
            if sum > random_stake {
                return i;
            }
        }
        panic!("Wrong StakeDistribution. Stake does not add up to 1.");
    }
}

// In a UniformStakeDistribution, all nodes have an equal amount of stake.
pub struct UniformStakeDistribution(usize);

impl UniformStakeDistribution {
    pub fn new(num_nodes: usize) -> Self {
        Self(num_nodes)
    }
}

impl StakeDistribution for UniformStakeDistribution {
    fn stake_of(&self, node: usize) -> f64 {
        if node >= self.0 {
            0.0
        } else {
            1.0 / self.0 as f64
        }
    }

    fn num_nodes(&self) -> usize {
        self.0
    }
}

// An ExponentialStakeDistribution uses a parameter 0 > p > 1, such that the first node (node 0)
// has a fraction p of the stake, the next node (node 1) has a fraction p of the rest, etc.
// The last node, however, is assigned all the remaining stake that has not yet been assigned to
// previous nodes, making this distribution not truly exponential in the mathematical sense (that
// would only work for an infinite number of nodes).
pub struct ExponentialStakeDistribution {
    num_nodes: usize,
    p: f64,
}

impl ExponentialStakeDistribution {
    pub fn new(num_nodes: usize, p: f64) -> Self {
        if (1.0-p).powi((num_nodes - 1) as i32) > p {
            println!("Warning: Tail of the exponential stake distribution heavier than first element.");
            println!("First element: {}", p);
            println!("Tail: {}", (1.0-p).powi((num_nodes - 1) as i32));
            println!("Assigning the whole tail to the last node anyway.");
            println!();
        }
        Self { num_nodes, p }
    }
}

impl StakeDistribution for ExponentialStakeDistribution {
    fn stake_of(&self, node: usize) -> f64 {
        if node >= self.num_nodes {
            0.0
        } else if node == self.num_nodes - 1 {
            (1.0-self.p).powi(node as i32) // Assign the whole tail to the last node.
        } else {
            (1.0-self.p).powi(node as i32) * self.p
        }
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }
}

pub struct CustomStakeDistribution {
    stakes: Vec<f64>
}

// The CustomStakeDistribution implements a lookup table with an arbitrary distribution of the
// stake. It can be loaded from a file containing one integer per line, where the k-th line
// represents the amount of stake of the k-th node (numbering starting from zero).
// Upon initialization, all stakes are normalized to be expressed by floating point numbers that
// sum up to one.
impl CustomStakeDistribution {
    pub fn from_file(file_name: &str) -> Self {
        // Read all lines of the input file.
        let file_content = fs::read_to_string(file_name).unwrap();
        let lines : Vec<String> = file_content.lines().map(|l| l.to_owned()).collect();

        // Convert string representations of stake to BigInt numbers and calculate total stake.
        let mut stakes_big = vec![];
        for line in lines {
            stakes_big.push(BigInt::from_str(line.as_str()).unwrap());
        }
        let total_stake : BigInt = stakes_big.iter().sum();

        // Express each node's stake as a fraction of the total stake.
        let mut stakes = vec![];
        for stake in stakes_big {
            stakes.push(BigRational::new(stake, total_stake.clone()).to_f64().unwrap());
        }

        Self{stakes}
    }
}

impl StakeDistribution for CustomStakeDistribution {
    fn stake_of(&self, node: usize) -> f64 {
        if node >= self.stakes.len() {
            0.0
        } else {
            self.stakes[node]
        }
    }

    fn num_nodes(&self) -> usize {
        self.stakes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn exponential_stake_distribution_0_5() {
        let distribution = ExponentialStakeDistribution::new(4, 0.5);
        assert_eq!(distribution.num_nodes(), 4);
        assert!(approx_equal(distribution.stake_of(0), 0.5));
        assert!(approx_equal(distribution.stake_of(1), 0.25));
        assert!(approx_equal(distribution.stake_of(2), 0.125));
        assert!(approx_equal(distribution.stake_of(3), 0.125));
    }

    #[test]
    fn exponential_stake_distribution_0_8() {
        let distribution = ExponentialStakeDistribution::new(4, 0.8);
        assert_eq!(distribution.num_nodes(), 4);
        assert!(approx_equal(distribution.stake_of(0), 0.8));
        assert!(approx_equal(distribution.stake_of(1), 0.2*0.8));
        assert!(approx_equal(distribution.stake_of(2), 0.2*0.2*0.8));
        assert!(approx_equal(distribution.stake_of(3), 0.2*0.2*0.2));
    }
}