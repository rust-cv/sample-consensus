#![no_std]

/// A model is a best-fit of at least some of the underlying data. You can compute residuals in respect to the model.
pub trait Model<Data> {
    /// Note that the residual error is returned as a 64-bit float. This allows the residual to be used for things
    /// other than sample consensus, such as optimization problems. For sample consensus, the residual should
    /// only be used to ensure it is within a threshold that roughly distinguishes inliers from outliers.
    ///
    /// The returned residual should always be positive, with a lower residual being associated with higher
    /// probability of being an inlier rather than an outlier.
    fn residual(&self, data: &Data) -> f64;
}

/// An `Estimator` is able to create a model that best fits a set of data.
/// It is also able to determine the residual error each data point contributes in relation to the model.
pub trait Estimator<Data> {
    /// `Model` is the model which is estimated from the underlying data
    type Model: Model<Data>;
    /// Iterator over the models produced from the data.
    type ModelIter: IntoIterator<Item = Self::Model>;

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
    fn estimate<I>(&self, data: I) -> Self::ModelIter
    where
        I: Iterator<Item = Data> + Clone;
}

/// A consensus algorithm extracts a consensus from an underlying model of data.
/// This consensus includes a model of the data and which datapoints fit the model.
///
/// Note that all the consensus methods take a `&mut self`. This allows the consensus to store
/// state such as an RNG or pre-allocated memory. This means multiple threads will be forced
/// to create their own `Consensus` instance, which is most efficient.
pub trait Consensus<E, Data>
where
    E: Estimator<Data>,
{
    /// Iterator over the indices of the inliers in the clonable iterator.
    type Inliers: IntoIterator<Item = usize>;

    /// Takes a slice over the data and an estimator instance.
    /// It returns `None` if no valid model could be found for the data and
    /// `Some` if a model was found.
    fn model<I>(&mut self, estimator: &E, data: I) -> Option<E::Model>
    where
        I: Iterator<Item = Data> + Clone;

    /// Takes a slice over the data and an estimator instance.
    /// It returns `None` if no valid model could be found for the data and
    /// `Some` if a model was found. It includes the inliers consistent with the model.
    fn model_inliers<I>(&mut self, estimator: &E, data: I) -> Option<(E::Model, Self::Inliers)>
    where
        I: Iterator<Item = Data> + Clone;
}

/// See [`Consensus`]. A multi-consensus can handle situations where different subsets of the data are consistent
/// with different models. This kind of consensus also considers whether a point is part of another orthogonal
/// model that is known before assuming it is a true outlier. In this situation there are inliers of different
/// models and then true outliers that are actual erroneous data that should be filtered out.
pub trait MultiConsensus<E, Data>
where
    E: Estimator<Data>,
{
    /// Iterator over the indices of the inliers in the clonable iterator.
    type Inliers: IntoIterator<Item = usize>;
    type Models: IntoIterator<Item = (E::Model, Self::Inliers)>;

    /// Takes a slice over the data and an estimator instance.
    /// It returns an iterator over all of the models and all of the inliers
    /// that are consistent with that model. Every point that is not an
    /// inlier of a given model is considered an outlier of that model.
    fn models<I>(&mut self, estimator: &E, data: I) -> Self::Models
    where
        I: Iterator<Item = Data> + Clone;
}
