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