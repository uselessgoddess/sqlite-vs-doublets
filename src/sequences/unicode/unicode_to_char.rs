use doublets::doublets::data::{AddrToRaw, RawToAddr};
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::num::LinkType;
use num_traits::AsPrimitive;

pub struct UnicodeToChar<T: LinkType> {
    pub(crate) unicode_marker: T,
}

impl<T: LinkType> UnicodeToChar<T> {
    pub fn new(unicode_marker: T) -> Self {
        Self { unicode_marker }
    }

    // TODO: impl `Converter`
    pub fn convert<L: ILinks<T>>(&self, links: &L, symbol: T) -> char {
        let link = links.get_link(symbol).unwrap();
        assert!(link.target == self.unicode_marker);
        RawToAddr::new()
            .convert(link.source)
            .to_u32()
            .map(|c| char::from_u32(c))
            .flatten()
            .unwrap()
    }
}
