
use std::default::default;
use std::marker::PhantomData;
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::num::LinkType;
use num_traits::zero;
use crate::sequences::walkers::SequenceWalker;

pub struct LeveledWalker<T, F> {
    is_element: F,
    _phantom: PhantomData<T>,
}

const CAPACITY: usize = 1024;

impl<T, F> LeveledWalker<T, F>
where
    T: LinkType + std::fmt::Display,
    F: Fn(T) -> bool
{
    pub fn with_pred( pred: F) -> Self {
        Self { is_element: pred, _phantom: PhantomData }
    }

    fn is_element(&self, element: T) -> bool {
        (self.is_element)(element)
    }

    fn count_filled(array: &Vec<T>) -> usize {
        array.iter().filter(|x| **x != zero()).count()
    }

    fn copy_filled(array: &Vec<T>) -> Vec<T> {
        array.iter().filter(|x| **x != zero()).map(|e| *e).collect()
    }

    fn to_vec<L: ILinks<T>>(&self, links: &L , sequence: T) -> Vec<T> {
        let mut len = 1;
        let mut has = false;
        let mut array = vec![sequence];
        if self.is_element(sequence) {
           return array;
        }
        loop {
            len *= 2;
            let mut vec = Vec::new();
            vec.resize(len, zero());
            has = false;
            for i in 0..array.len() {
                let candidate = array[i];
                if candidate.is_zero() {
                    continue;
                }
                let double_offset = i * 2;
                if self.is_element(candidate) {
                    vec[double_offset] = candidate;
                } else {
                    let link = links.get_link(candidate).unwrap(); // TODO: unwrap
                    let source = link.source;
                    let target = link.target;
                    vec[double_offset] = source;
                    vec[double_offset + 1] = target;
                    if !has {
                        has = !(self.is_element(source) && self.is_element(target));
                    }
                }
            }

            array = vec;

            if !has {
                break;
            }
        }

        if Self::count_filled(&array) == array.len() {
            array
        } else {
            // TODO: println!("{:?}", array);
            Self::copy_filled(&array)
        }
    }
}

impl<T, F> SequenceWalker<T> for LeveledWalker<T, F>
where
    T: LinkType + std::fmt::Display,
    F: Fn(T) -> bool
{
    fn walk<L: ILinks<T>>(&self, links: &L , sequence: T) -> Vec<T> {
        self.to_vec(links, sequence)
    }
}
