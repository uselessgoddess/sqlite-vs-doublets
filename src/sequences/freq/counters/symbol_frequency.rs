use std::collections::VecDeque;
use doublets::num::LinkType;
use std::default::default;
use doublets::doublets::{ILinks, ILinksExtensions};
use num_traits::one;
use crate::sequences::freq::counters::Counter;

pub struct StoppableWalker;

impl StoppableWalker {
    pub fn walk_right<T, FS, FT, FE, FV>(
        sequence: T,
        get_source: FS,
        get_target: FT,
        is_element: FE,
        mut visit: FV,
    ) -> bool
        where
            T: LinkType,
            FS: Fn(T) -> T,
            FT: Fn(T) -> T,
            FE: Fn(T) -> bool,
            FV: FnMut(T) -> bool,
    {
        let mut stack = VecDeque::new();
        let mut element = sequence;
        if is_element(element) {
            return visit(element);
        }

        loop {
            if is_element(element) {
                if stack.len() == 0 {
                    return true;
                }
                element = stack.pop_back().unwrap(); // TODO: 100% `Some`
                let source = get_source(element);
                let target = get_target(element);
                if is_element(source) && !visit(source) || is_element(target) && !visit(target) {
                    return false;
                }
                element = target;
            } else {
                stack.push_back(element);
                element = get_source(element);
            }
        }
    }

    pub fn walk_left<T, FS, FT, FE, FV>(
        sequence: T,
        get_source: FS,
        get_target: FT,
        is_element: FE,
        mut visit: FV,
    ) -> bool
        where
            T: LinkType,
            FS: Fn(T) -> T,
            FT: Fn(T) -> T,
            FE: Fn(T) -> bool,
            FV: FnMut(T) -> bool,
    {
        let mut stack = VecDeque::new();
        let mut element = sequence;
        if is_element(element) {
            return visit(element);
        }

        loop {
            if is_element(element) {
                if stack.len() == 0 {
                    return true;
                }
                element = stack.pop_back().unwrap(); // TODO: 100% `Some`
                let source = get_source(element);
                let target = get_target(element);
                if is_element(target) && !visit(target) || is_element(source) && !visit(source) {
                    return false;
                }
                element = target;
            } else {
                stack.push_back(element);
                element = get_target(element);
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct SymbolFreq<T: LinkType> {
    pub sequence: T,
    pub symbol: T,
    pub total: T,
}

impl<T: LinkType> SymbolFreq<T> {
    pub fn new(sequence: T, symbol: T) -> Self {
        Self {
            sequence,
            symbol,
            total: default(),
        }
    }
}

impl<T: LinkType> Counter<T> for SymbolFreq<T> {
    fn count<Links: ILinks<T>>(&mut self, links: &Links, _: ()) -> T {
        if self.total != default() {
            self.total
        } else {
            let mut total = default();
            StoppableWalker::walk_right(
                self.sequence,
                |index| links.get_link(index).unwrap().source, // TODO: expect error
                |index| links.get_link(index).unwrap().target, // TODO: expect error
                |index| index == self.symbol || links.is_partial_point(index),
                |index| { if index == self.symbol { total = total + one(); } true }
            );
            self.total = total;
            return self.total;
        }
    }
}
