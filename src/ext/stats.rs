use std::ops::Add;

pub struct VecStats<Iter>
where
    Iter: Iterator,
{
    iter: Iter,
}

impl<Iter, T> VecStats<Iter>
where
    Iter: ExactSizeIterator<Item = T>,
    T: Add<Output = T> + Into<f64>,
{
    pub fn mean(self) -> Option<f64> {
        let n: f64 = self.iter.len() as f64;
        let total = self.iter.reduce(|a, b| a + b);

        total.map(|total| total.into() / n as f64)
    }
}

pub trait VecStatsExt<Iter>
where
    Iter: Iterator,
{
    fn stats(self) -> VecStats<Iter>;
}

impl<Iter> VecStatsExt<Iter> for Iter
where
    Iter: Iterator,
{
    fn stats(self) -> VecStats<Iter> {
        VecStats { iter: self }
    }
}
