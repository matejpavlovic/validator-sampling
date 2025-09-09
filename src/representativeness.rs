use rand_distr::Binomial;
use num_rational::BigRational;
use num_integer::binomial;
use num_bigint::BigInt;
use rand::distr::Distribution;
use rand_distr::num_traits::{FromPrimitive, One, ToPrimitive, Zero};

// Repeatedly samples a binomially distributed random variable and returns
// how many times (relative to the total number of samples) the value exceeded a given
// fault_threshold parameter. The random variable represents the number of faulty validators
// in a validator sample and the fault threshold expresses the tolerated number of faulty
// validators.
pub fn simulate_prob(n: u64, p: f64, fault_threshold: f64, iterations: usize) -> f64 {
    let mut rng = rand::rng();
    let bin = Binomial::new(n, p).unwrap();

    let mut failures = 0;
    for _ in 0..iterations {
        let k: u64 = bin.sample(&mut rng);
        let failure_rate : f64 = (k as f64) / (n as f64);
        if failure_rate > fault_threshold {
            failures += 1;
       }
    }

    (failures as f64) / (iterations as f64)
}

// Probability mass function of the binomial distribution Bin(n, p),
// evaluated at k. Returns the probability that,
// out of n Bernoulli trials wit success probability p,
// there will be exactly k successes.
fn bin_probability_mass(n: u32, p: &BigRational, k: u32) -> BigRational {
    if k > n {
        return BigRational::from_i32(0).unwrap();
    }

    let bin_coeff = binomial(BigInt::from(n), BigInt::from(k));

    BigRational::from(bin_coeff) * p.pow(k as i32) * (BigRational::one() - p).pow((n-k) as i32)
}

// Binomial cumulative distribution function. Returns the probability that,
// out of n Bernoulli trials with success probability p,
// there will be at most k successes.
fn bin_probability_cdf(n: u32, p: &BigRational, k: u32) -> BigRational {
    let mut result = BigRational::zero();


    for i in 0..=k {
        result += bin_probability_mass(n, p, i);
    }

    result
}

// Returns the probability that a validator sample of a given size, taken from a set of nodes with
// a fraction f (expressed a rational number f_num/f_denom) contains a fraction of at least
// fault_threshold faulty validators.
pub fn failure_probability(sample_size: u32, f_num: u32, f_denom: u32, fault_threshold: f64) -> BigRational {
    let f = BigRational::new(BigInt::from(f_num), BigInt::from(f_denom));
    let max_faults = ((sample_size as f64) * fault_threshold).floor() as u32;

    BigRational::one() - &bin_probability_cdf(sample_size, &f, max_faults)
}

// Returns the minimal sample size smaller than max_sample_size for which the failure probability
// (as computed by the failure_probability function) is lower than max_prob. f_num, f_denom, and
// fault_threshold are passed directly to failure_probability. If no sample size up to
// max_sample_size satisfies this condition, returns None.
pub fn min_sample_size(f_num: u32, f_denom: u32, fault_threshold: f64, max_prob: f64, max_sample_size: u32) -> Option<u32> {
    for n in 1..=max_sample_size {
        let prob = failure_probability(n, f_num, f_denom, fault_threshold).to_f64().unwrap();
        if prob < max_prob {
            return Some(n);
        }
    }

    None
}

pub fn compute() {
    let sim_iterations = 100000;

    println!();
    println!("================================================================================");
    println!("SAMPLE REPRESENTATIVENESS");
    println!("I.e.: What is the probability of a sample containing too many faulty validators?");
    println!("================================================================================");
    println!();

    println!("Finding smallest sample size that is safe for f = 1/3 with failure probability 1.9e-13.");
    if let Some(sample_size) = min_sample_size(1, 3, 2.0/3.0, 1.9e-13, 200) {
        println!("Minimum sample size: {}", sample_size);
    } else {
        println!("No sample size is safe under given parameters.");
    }
    println!("Finding smallest sample size that is safe for f = 1/3 with failure probability 1.9e-16.");
    if let Some(sample_size) = min_sample_size(1, 3, 2.0/3.0, 1.9e-16, 200) {
        println!("Minimum sample size: {}", sample_size);
    } else {
        println!("No sample size is safe under given parameters.");
    }
    println!("Finding smallest sample size that is live for f = 1/5 with failure probability 1e-3.");
    if let Some(sample_size) = min_sample_size(1, 5, 1.0/3.0, 1e-3, 200) {
        println!("Minimum sample size: {}", sample_size);
    } else {
        println!("No sample size is safe under given parameters.");
    }
    println!();
    println!("Probabilities of samples being safe and live, for various parameters.");
    println!("n: sample size");
    println!("f: assumed system-wide fraction of stake controlled by malicious nodes");
    println!("A sample not safe if at least 2/3 of its nodes are faulty.");
    println!("A sample not live if at least 1/3 of its nodes are faulty.");
    println!();
    println!(" ______________________________________________ ");
    println!("|      n |      f | not safe | not live | type |");
    println!(" ---------------------------------------------- ");
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} | Calc |",
             141,
             "1/3",
             failure_probability(141, 1, 3, 2.0/3.0).to_f64().unwrap(),
             failure_probability(141, 1, 3, 1.0/3.0).to_f64().unwrap(),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} |  Sim |",
             141,
             "1/3",
             simulate_prob(141, 1.0/3.0, 2.0/3.0, sim_iterations),
             simulate_prob(141, 1.0/3.0, 1.0/3.0, sim_iterations),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} | Calc |",
             111,
             "1/3",
             failure_probability(111, 1, 3, 2.0/3.0).to_f64().unwrap(),
             failure_probability(111, 1, 3, 1.0/3.0).to_f64().unwrap(),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} |  Sim |",
             111,
             "1/3",
             simulate_prob(111, 1.0/3.0, 2.0/3.0, sim_iterations),
             simulate_prob(111, 1.0/3.0, 1.0/3.0, sim_iterations),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} | Calc |",
             111,
             "1/5",
             failure_probability(111, 1, 5, 2.0/3.0).to_f64().unwrap(),
             failure_probability(111, 1, 5, 1.0/3.0).to_f64().unwrap(),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} |  Sim |",
             111,
             "1/5",
             simulate_prob(111, 0.2, 2.0/3.0, sim_iterations),
             simulate_prob(111, 0.2, 1.0/3.0, sim_iterations),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} | Calc |",
             90,
             "1/5",
             failure_probability(90, 1, 5, 2.0/3.0).to_f64().unwrap(),
             failure_probability(90, 1, 5, 1.0/3.0).to_f64().unwrap(),
    );
    println!("| {:6} | {:>6} | {:8.2e} | {:8.2e} |  Sim |",
             90,
             "1/5",
             simulate_prob(90, 0.2, 2.0/3.0, sim_iterations),
             simulate_prob(90, 0.2, 1.0/3.0, sim_iterations),
    );
    println!(" ---------------------------------------------- ");
}