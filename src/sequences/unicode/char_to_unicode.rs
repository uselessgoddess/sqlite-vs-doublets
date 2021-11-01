use doublets::doublets::data::AddrToRaw;
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::num::LinkType;
use num_traits::AsPrimitive;

pub struct CharToUnicode<T: LinkType> {
    unicode_marker: T,
}

impl<T: LinkType> CharToUnicode<T> {
    pub fn new(unicode_marker: T) -> Self {
        Self { unicode_marker }
    }

    // TODO: impl `Converter`
    pub fn convert<L: ILinks<T>>(&self, links: &mut L, symbol: char) -> T {
        let unary = T::from(AddrToRaw::new().convert(symbol as usize)).unwrap(); // TODO: Converter
        links.get_or_create(unary, self.unicode_marker)
    }
}
