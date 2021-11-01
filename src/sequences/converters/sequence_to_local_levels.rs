use doublets::doublets::{Doublet, ILinks};
use doublets::num::LinkType;
use crate::sequences::converters::links_to_sequence_base::LinksToSequence;
use crate::sequences::freq::{Counter, FreqToNum};
use std::default::default;

pub struct SequenceToLocalLevels<T: LinkType, C: Counter<T, T>> {
    converter: FreqToNum<T, C>,
}

impl<T: LinkType, C: Counter<T, T>> SequenceToLocalLevels<T, C> {
    pub fn new(converter: FreqToNum<T, C>) -> Self {
        Self { converter }
    }

    // TODO: create `Converter` trait
    fn convert<L, Links>(&mut self, source: L) -> Vec<T>
        where
            L: IntoIterator<Item = T, IntoIter: ExactSizeIterator>,
    {
        let sequence: Vec<_> = source.into_iter().collect();
        assert!(sequence.len() >= 2, "{}", todo!("add message"));

        let mut levels = Vec::new();
        levels.resize(sequence.len(), default());
        for i in 1..sequence.len() - 1 {
            let prev = self.converter.convert(Doublet::new(sequence[i - 1], sequence[i]));
            let next = self.converter.convert(Doublet::new(sequence[i], sequence[i + 1]));
            levels[i] = prev.max(next);
        }
        // TODO: 100% `Some`
        let len = sequence.len();
        let last_1 = sequence[len - 1];
        let last_2 = sequence[len - 2];
        *levels.last_mut().unwrap() = self.converter.convert(Doublet::new(last_2, last_1));
        return levels;
    }
}
