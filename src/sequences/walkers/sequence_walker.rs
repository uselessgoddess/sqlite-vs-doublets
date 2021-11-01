use doublets::doublets::ILinks;
use doublets::num::LinkType;

pub trait SequenceWalker<T: LinkType>
{
    fn walk<L: ILinks<T>>(&self, links: &L , sequence: T) -> Vec<T>;
}
