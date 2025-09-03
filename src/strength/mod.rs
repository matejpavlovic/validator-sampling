mod stake_distribution;
mod validator_sample;
mod simulation;

use stake_distribution::*;
use simulation::Simulation;

pub fn simulate() {
    let num_nodes = 296;
    let sample_size = 111;
    let exp_distribution_p = 0.1;
    let actual_distribution_file = "node-stakes-september-2025";
    let iterations = 1000;

    println!();
    println!("================================================================================");
    println!("SAMPLE STRENGTH");
    println!("I.e.: What is the cost of dynamically corrupting a validator sample?");
    println!("================================================================================");
    println!();

    println!("Number of nodes: {}", num_nodes);
    println!("Sample size: {}", sample_size);
    println!("System-wide total stake: 1.0");
    println!("Simulation iterations for each distribution: {}", iterations);
    println!();
    let uniform_distribution = UniformStakeDistribution::new(num_nodes);
    let mut simulation = Simulation::new(&uniform_distribution);
    simulation.run(sample_size, iterations);
    let (strong_quorum_avg, strong_quorum_std_dev) = simulation.strong_quorum_stake();
    let (weak_quorum_avg, weak_quorum_std_dev) = simulation.weak_quorum_stake();
    let (total_avg, total_std_dev) = simulation.total_stakes();
    println!("Uniform distribution:");
    println!("Weak quorum stake: avg {} (std-dev {})", weak_quorum_avg, weak_quorum_std_dev);
    println!("Strong quorum stake: avg {} (std-dev {})", strong_quorum_avg, strong_quorum_std_dev);
    println!("Total stake: avg {} (std-dev {})", total_avg, total_std_dev);
    println!();
    let exponential_distribution = ExponentialStakeDistribution::new(num_nodes, exp_distribution_p);
    let mut simulation = Simulation::new(&exponential_distribution);
    simulation.run(sample_size, iterations);
    let (strong_quorum_avg, strong_quorum_std_dev) = simulation.strong_quorum_stake();
    let (weak_quorum_avg, weak_quorum_std_dev) = simulation.weak_quorum_stake();
    let (total_avg, total_std_dev) = simulation.total_stakes();
    println!("Pseudo-exponential distribution (p = {exp_distribution_p}):");
    println!("Weak quorum stake: avg {} (std-dev {})", weak_quorum_avg, weak_quorum_std_dev);
    println!("Strong quorum stake: avg {} (std-dev {})", strong_quorum_avg, strong_quorum_std_dev);
    println!("Total stake: avg {} (std-dev {})", total_avg, total_std_dev);
    println!();
    let actual_distribution = CustomStakeDistribution::from_file(actual_distribution_file);
    let mut simulation = Simulation::new(&actual_distribution);
    simulation.run(sample_size, iterations);
    let (strong_quorum_avg, strong_quorum_std_dev) = simulation.strong_quorum_stake();
    let (weak_quorum_avg, weak_quorum_std_dev) = simulation.weak_quorum_stake();
    let (total_avg, total_std_dev) = simulation.total_stakes();
    println!("Actual current distribution ({} nodes in file '{}'):", actual_distribution.num_nodes(), actual_distribution_file);
    println!("Weak quorum stake: avg {} (std-dev {})", weak_quorum_avg, weak_quorum_std_dev);
    println!("Strong quorum stake: avg {} (std-dev {})", strong_quorum_avg, strong_quorum_std_dev);
    println!("Total stake: avg {} (std-dev {})", total_avg, total_std_dev);
}