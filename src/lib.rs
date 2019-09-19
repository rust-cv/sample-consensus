#![no_std]

/// A model is a best-fit of at least some of the underlying data. You can compute residuals in respect to the model.
pub trait Model {
    type Data;

    /// Note that the residual error is returned as a 32-bit float. This might be harder to preserve precision with
    /// than a 64-bit float, but it will be faster to perform RANSAC if care is taken to avoid
    /// [round-off error](https://en.wikipedia.org/wiki/Round-off_error)
    /// using [Kahan's algorithm](https://en.wikipedia.org/wiki/Kahan_summation_algorithm) or using
    /// [Pairwise summation](https://en.wikipedia.org/wiki/Pairwise_summation). If the number of datapoints is
    /// small, then there should be little issue with the accumulation of
    /// [round-off error](https://en.wikipedia.org/wiki/Round-off_error) and 32-bit floats should work
    /// without any concern.
    ///
    /// Here are some helpers to allow you to perform less lossy summation:
    ///
    /// - [Kahan Summation](https://crates.io/crates/kahan)
    /// - [Pairwise Summation](https://docs.rs/itertools/0.8.0/itertools/trait.Itertools.html#method.tree_fold1)
    ///     - `let sum = estimator.residuals(data).tree_fold1(|a, b| a + b).unwrap_or(0.0)`
    ///
    /// If all you wish to do is filter data points out if they are above a certian threshold of error
    /// then the 32-bit float's precision will be no issue for you.
    fn residual(&self, data: Self::Data) -> f32;
}

/// An `Estimator` is able to create a model that best fits a set of data.
/// It is also able to determine the residual error each data point contributes in relation to the model.
pub trait Estimator {
    /// The data for which the model is being estimated.
    type Data;

    /// `Model` is the model which is estimated from the underlying data
    type Model;

    /// The minimum number of samples that the estimator can estimate a model from.
    const MIN_SAMPLES: usize;

    /// Takes in an iterator over the data and produces a model that best fits the data.
    ///
    /// This must be passed at least `Self::MIN_SAMPLES` data points, otherwise `estimate` should panic
    /// to indicate a developer error.
    ///
    /// `None` should be returned only if a model is impossible to estimate based on the data.
    /// For instance, if a particle has greater than infinite mass, a point is detected behind a camera,
    /// an equation has an imaginary answer, or non-causal events happen, then a model may not be produced.
    fn estimate<I>(&self, data: I) -> Option<Self::Model>
    where
        I: Iterator<Item = Self::Data>;
}

/// A consensus algorithm extracts a consensus from an underlying model of data.
pub trait Consensus<D, I, E>
where
    I: Iterator<Item = D> + Clone,
    E: Estimator<Data = D>,
{
    /// See [`Estimator::estimate`] for more details on when a model is returned or not.
    ///
    /// This takes in an estimator and a clonable iterator over the data.
    ///
    /// It takes a mutable reference to self to allow it to consume randomness from an RNG.
    fn estimate<R>(&mut self, estimator: &E, data: I) -> Option<E::Model>;
}
