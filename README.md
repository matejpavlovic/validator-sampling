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

The following is the output of the code at its initial version (first commit).
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
Weak quorum stake: avg 0.1175979729729729 (std-dev 0.004433587472642292)
Strong quorum stake: avg 0.22096621621621595 (std-dev 0.008233201060193967)
Total stake: avg 0.3127263513513517 (std-dev 0.011157049157700284)

Pseudo-exponential distribution (p = 0.1):
Weak quorum stake: avg 0.2665317040912626 (std-dev 0.042213623723942544)
Strong quorum stake: avg 0.6130588760781609 (std-dev 0.05428436467536695)
Total stake: avg 0.9145778940781613 (std-dev 0.02230139115405164)

Actual current distribution (296 nodes in file 'node-stakes-september-2025'):
Weak quorum stake: avg 0.11404335827767918 (std-dev 0.02255769155208962)
Strong quorum stake: avg 0.39008585943603474 (std-dev 0.04063429370796081)
Total stake: avg 0.7054378224682029 (std-dev 0.028179610348842057)

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
A sample not unsafe if at least 2/3 of its nodes are faulty.
A sample not live if at least 2/3 of its nodes are faulty.

 ______________________________________________ 
|      n |      f | not safe | not live | type |
 ---------------------------------------------- 
|    141 |    1/3 | 1.64e-16 |  4.61e-1 | Calc |
|    141 |    1/3 |   0.00e0 |  4.64e-1 |  Sim |
|    111 |    1/3 | 1.88e-13 |  4.56e-1 | Calc |
|    111 |    1/3 |   0.00e0 |  4.56e-1 |  Sim |
|    111 |    1/5 | 2.65e-27 |  3.20e-4 | Calc |
|    111 |    1/5 |   0.00e0 |  3.60e-4 |  Sim |
|     90 |    1/5 | 1.34e-22 |  9.66e-4 | Calc |
|     90 |    1/5 |   0.00e0 |  7.40e-4 |  Sim |
 ---------------------------------------------- 
```