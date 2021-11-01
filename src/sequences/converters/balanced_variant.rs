use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::num::LinkType;
use crate::sequences::converters::links_to_sequence_base::LinksToSequence;
use std::default::default;
use std::marker::PhantomData;

pub struct BalancedVariant<T: LinkType> {
    _phantom: PhantomData<T>,
}

impl<T: LinkType> BalancedVariant<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    fn halve_sequnce<Links>(
        &mut self,
        links: &mut Links,
        destination: &mut Vec<T>,
        source: &Vec<T>,
        len: usize,
    ) where
        Links: ILinks<T>,
    {
        let loop_len = len - (len % 2);
        for i in (0..loop_len).step_by(2) {
            destination[i / 2] = ILinksExtensions::get_or_create(links, source[i], source[i + 1]);
        }
        if len > loop_len {
            destination[len / 2] = source[len - 1];
        }
    }
}

impl<T: LinkType> LinksToSequence<T> for BalancedVariant<T> {
    fn convert<L, Links>(&mut self, links: &mut Links, sequence: L) -> T
    where
        L: IntoIterator<Item = T, IntoIter: Iterator>,
        Links: ILinks<T>,
    {
        let sequence = sequence.into_iter();
        let mut sequence: Vec<_> = sequence.collect();
        let mut len = sequence.len();
        if len < 1 {
            return default();
        }

        if len == 1 {
            return sequence[0];
        }

        if len > 2 {
            let mut halved = Vec::<T>::new();
            halved.resize((len / 2) + (len % 2), default());
            self.halve_sequnce(links, &mut halved, &sequence, len);
            len = halved.len();
            sequence = halved;
        }

        while len > 2 {
            // TODO: fix performance
            let clone = sequence.clone();
            self.halve_sequnce(links, &mut sequence, &clone, len);
            len = (len / 2) + (len % 2);
        }
        return links.get_or_create(sequence[0], sequence[1]);
    }
}
