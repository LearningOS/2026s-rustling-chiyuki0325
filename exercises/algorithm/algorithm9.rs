/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: Vec::<T>::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        if self.items.len() > 1 {
            self.float_up(self.items.len() - 1);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.len()
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
    }

    fn child_to_swap(&self, idx: usize) -> Option<usize> {
        let l = self.left_child_idx(idx);
        let r = self.right_child_idx(idx);

        if l >= self.len() {
            None
        } else if r >= self.len() {
            Some(l)
        } else if (self.comparator)(&self.items[l], &self.items[r]) {
            Some(l)
        } else {
            Some(r)
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.items.swap(a, b);
    }

    fn float_up(&mut self, idx: usize) {
        let mut i = idx;
        while i > 0 {
            let pa = self.parent_idx(i);
            if (self.comparator)(&self.items[pa], &self.items[i]) {
                break;
            }
            self.swap(i, pa);
            i = pa;
        }
    }
    fn sink_down(&mut self, idx: usize) {
        let mut i = idx;
        loop {
            if let Some(mc) = self.child_to_swap(i) {
                if !(self.comparator)(&self.items[mc], &self.items[i]) {
                    break;
                }
                self.swap(i, mc);
                i = mc;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }

        let last_idx = self.items.len() - 1;
        self.swap(0, last_idx);

        let res = self.items.pop();
        
        if !self.items.is_empty() {
            self.sink_down(0);
        }

        res
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
