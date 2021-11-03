use std::collections::HashMap;
use doublets::doublets::data::AddrToRaw;
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::num::LinkType;
use crate::sequences::converters::balanced_variant::BalancedVariant;
use crate::sequences::converters::links_to_sequence_base::LinksToSequence;
use crate::sequences::unicode::char_to_unicode::CharToUnicode;

pub struct StringToUnicode<T: LinkType> {
    sequence_marker: T,
    to_unicode: CharToUnicode<T>,
    map: HashMap<String, T>,
}

impl<T: LinkType> StringToUnicode<T> {
    pub fn new(to_unicode: CharToUnicode<T>, sequence_marker: T) -> Self {
        Self { sequence_marker, to_unicode, map: Default::default() }
    }

    // TODO: impl `Converter`
    pub fn convert<L: ILinks<T>>(&mut self, links: &mut L, string: String) -> T {
        *self.map.entry(string)
            .or_insert_with_key(|string| {
                let seq: Vec<_> = string.chars()
                    .map(|c| self.to_unicode.convert(links, c))
                    .collect();

                let sequence = BalancedVariant::new().convert(links, seq);
                links.get_or_create(sequence, self.sequence_marker)
            })
    }
}
