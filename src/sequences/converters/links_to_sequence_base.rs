use doublets::doublets::ILinks;
use doublets::num::LinkType;

pub trait LinksToSequence<T: LinkType> {
    fn convert<L, Links>(&mut self, links: &mut Links, source: L) -> T
    where
        L: IntoIterator<Item=T, IntoIter: Iterator>,
        Links: ILinks<T>;
}
