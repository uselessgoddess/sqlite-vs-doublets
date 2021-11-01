use std::collections::{HashMap, LinkedList};
use doublets::doublets::data::AddrToRaw;
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::num::LinkType;
use crate::sequences::converters::balanced_variant::BalancedVariant;
use crate::sequences::converters::links_to_sequence_base::LinksToSequence;
use crate::sequences::unicode::UnicodeToChar;
use crate::sequences::walkers::{LeveledWalker, SequenceWalker};

pub struct UnicodeToString<T: LinkType> {
    sequence_marker: T,
    to_char: UnicodeToChar<T>,
    
    map: HashMap<T, String>,
}

impl<T: LinkType> UnicodeToString<T> {
    pub fn new(to_char: UnicodeToChar<T>, sequence_marker: T) -> Self {
        Self { sequence_marker, to_char, map: Default::default() }
    }

    // TODO: impl `Converter`
    pub fn convert<L: ILinks<T>>(&mut self, links: &L, source: T) -> String {
        self.map.entry(source)
            .or_insert_with_key(|source| -> String {
                let source = source.clone();
                let link = links.get_link(source).unwrap();
                assert!(link.target == self.sequence_marker);

                let sequence = link.source;
                let walker = LeveledWalker::with_pred(|c| {
                    links.is_partial_point(c) || {
                        let link = links.get_link(c).unwrap();
                        link.target == self.to_char.unicode_marker
                    }
                });

                walker.walk(&*links, sequence).into_iter()
                    .map(|c| self.to_char.convert(links, c))
                    .collect()
            }).clone()
    }
}
