# sample-consensus

[![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![LoC][lo]

[ci]: https://img.shields.io/crates/v/sample-consensus.svg
[cl]: https://crates.io/crates/sample-consensus/

[li]: https://img.shields.io/crates/l/specs.svg?maxAge=2592000

[di]: https://docs.rs/sample-consensus/badge.svg
[dl]: https://docs.rs/sample-consensus/

[lo]: https://tokei.rs/b1/github/rust-photogrammetry/sample-consensus?category=code

`sample-consensus` provides abstractions for sample consensus algorithms such as RANSAC.

An example of how to use these abstractions is present in the [ARRSAC repository](https://github.com/rust-photogrammetry/arrsac).

This allows one to create a RANSAC algorithm (`Consensus` or `MultiConsensus`) that is independent of the underlying system.
You can also create a `Model` and an `Estimator` for different systems. An `Estimator` only needs to estimate a model
from a subset of some data. With this system, you can quickly define an `Estimator` based on an algorithm, like
the 8-point algorithm, and you don't have to worry about the details of how the sample consensus algorithm works. It will
just find a model that fits the data based on that estimation algorithm. Crates may exist that create instantiations
of any one of those three things.

The design of this system is directly and highly inspired by the information on [this page](http://theia-sfm.org/ransac.html)
from TheiaSfM. This crate deviates in some ways by avoiding the requirement of making any memory allocation so it can run on embedded.
It does this through the use of trait method type parameters so that the caller gets to choose how the data is provided.
A big thanks to Chris Sweeney for starting and maintaining TheiaSfM.
