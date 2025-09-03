# Analysis of PoS Validator Sampling

A validator sample is a set of nodes in a Proof-of-Stake (PoS) system picked randomly based on the nodes' stake.
The probability of selecting a node is proportional to the stake it has.
This code can be used to analyze the probability of a validator sample being corrupted, depending on parameters like
sample size and the fraction of stake controlled by the adversary.
Moreover, given different distributions of the stake over the nodes, the code shows (through simulation) what fraction
of the overall stake is backing critical quorums of the samples.

A detailed description of the sampling strategy and failure model can be found in the corresponding
[design document](https://docs.google.com/document/d/1cwrz5iqRhHvuzfpOJE4LW3heJEXfUFYwJp_gA29pZ0c/edit?usp=sharing).

## Running the code

In the main project directory, simply run

```shell
cargo run
```

For simplicity, the parameters are hard-coded, but they are easy to find in the code.

## Sample output

The following is the output of the code at its initial working version (third commit).
If run again, the output should be almost the same and only very slightly differ in the results of the simulation that
are probabilistic by nature.


```text
================================================================================
SAMPLE STRENGTH
I.e.: What is the cost of dynamically corrupting a validator sample?
================================================================================

Number of nodes: 296
Sample size: 111
System-wide total stake: 1.0
Simulation iterations for each distribution: 1000

Uniform distribution:
Weak quorum stake: avg 0.11745608108108098 (std-dev 0.004655069038187549)
Strong quorum stake: avg 0.22141891891891854 (std-dev 0.00848282805509339)
Total stake: avg 0.31367229729729756 (std-dev 0.011365071697223308)

Pseudo-exponential distribution (p = 0.1):
Weak quorum stake: avg 0.2664877925300539 (std-dev 0.04238627234783938)
Strong quorum stake: avg 0.6114667493554351 (std-dev 0.04977763941899594)
Total stake: avg 0.9147741573554351 (std-dev 0.02246831483248877)

Actual current distribution (296 nodes in file 'node-stakes-september-2025'):
Weak quorum stake: avg 0.11295979578104386 (std-dev 0.021716119858098515)
Strong quorum stake: avg 0.3866259583452415 (std-dev 0.03867148947252776)
Total stake: avg 0.7055378218060195 (std-dev 0.028826450590846254)

================================================================================
SAMPLE REPRESENTATIVENESS
I.e.: What is the probability of a sample containing too many faulty validators?
================================================================================

Finding smallest sample size that is safe for f = 1/3 with failure probability 1.9e-13.
Minimum sample size: 111
Finding smallest sample size that is safe for f = 1/3 with failure probability 1.9e-16.
Minimum sample size: 141
Finding smallest sample size that is live for f = 1/5 with failure probability 1e-3.
Minimum sample size: 90

Probabilities of samples being safe and live, for various parameters.
n: sample size
f: assumed system-wide fraction of stake controlled by malicious nodes
A sample not safe if at least 2/3 of its nodes are faulty.
A sample not live if at least 1/3 of its nodes are faulty.

 ______________________________________________ 
|      n |      f | not safe | not live | type |
 ---------------------------------------------- 
|    141 |    1/3 | 1.64e-16 |  4.61e-1 | Calc |
|    141 |    1/3 |   0.00e0 |  4.61e-1 |  Sim |
|    111 |    1/3 | 1.88e-13 |  4.56e-1 | Calc |
|    111 |    1/3 |   0.00e0 |  4.56e-1 |  Sim |
|    111 |    1/5 | 2.65e-27 |  3.20e-4 | Calc |
|    111 |    1/5 |   0.00e0 |  3.20e-4 |  Sim |
|     90 |    1/5 | 1.34e-22 |  9.66e-4 | Calc |
|     90 |    1/5 |   0.00e0 |  9.50e-4 |  Sim |
 ---------------------------------------------- 
```