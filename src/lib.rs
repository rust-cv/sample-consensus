#![no_std]

type EstimatorData<E> = <<E as Estimator>::Model as Model>::Data;

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
    /// `Model` is the model which is estimated from the underlying data
    type Model: Model;

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
        I: Iterator<Item = EstimatorData<Self>>;
}

/// A consensus algorithm extracts a consensus from an underlying model of data.
/// This consensus includes a model of the data and which datapoints fit each model.
/// It is entirely possible that some data points fit multiple models.
///
/// Note that all the consensus methods take a `&mut self`. This allows the consensus to store
/// state such as an RNG or pre-allocted memory. This means multiple threads will be forced
/// to create their own `Consensus` instance, which is most efficient.
pub trait Consensus<E>
where
    E: Estimator,
{
    /// Iterator over the indices of the inliers in the clonable iterator.
    type Inliers: Iterator<Item = usize>;
    /// Iterator over the models and their associated inliers.
    type ModelInliers: Iterator<Item = (E::Model, Self::Inliers)>;

    /// This takes in an estimator and a clonable iterator over the data.
    /// It returns `None` if no valid model could be found for the data and
    /// `Some` if a model was found.
    fn model<I>(&mut self, estimator: &E, data: I) -> Option<E::Model>
    where
        I: Iterator<Item = EstimatorData<E>> + Clone;

    /// This takes in an estimator and a clonable iterator over the data.
    /// It returns an iterator over all of the models and all of the inliers
    /// that are consistent with that model. Every point that is not an
    /// inlier of a given model is considered an outlier of that model.
    fn models<I>(&mut self, estimator: &E, data: I) -> Self::ModelInliers
    where
        I: Iterator<Item = EstimatorData<E>> + Clone;
}