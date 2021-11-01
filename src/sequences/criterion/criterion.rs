use doublets::doublets::ILinks;
use doublets::num::LinkType;

pub trait Criterion<T: LinkType> {
    // TODO: use implicit `Links` instead explicit of `&Links`
    fn is_match<Links: ILinks<T>>(&self, links: &Links, _: T) -> bool;
}
