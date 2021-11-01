use doublets::doublets::ILinks;
use doublets::num::LinkType;
use crate::sequences::converters::links_to_sequence_base::LinksToSequence;

pub trait Serialized<Conv: LinksToSequence<T>, T: LinkType> {
    fn serialize<Links: ILinks<T>>(&self, links: &mut Links);
}
