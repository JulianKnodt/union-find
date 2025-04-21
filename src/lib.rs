#![feature(let_chains)]

use std::cell::Cell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionFind {
    ptrs: Vec<Cell<usize>>,

    len: usize,
}

impl UnionFind {
    #[inline]
    pub fn new(size: usize) -> Self {
        let ptrs = vec![Cell::new(0); size];
        for (i, ptr) in ptrs.iter().enumerate() {
            ptr.set(i);
        }
        Self { ptrs, len: size }
    }
    #[inline]
    pub fn get(&self, mut v: usize) -> usize {
        if v >= self.ptrs.len() {
            return v;
        }
        while let n = unsafe { self.ptrs.get_unchecked(v).get() }
            && n != v
        {
            v = n;
        }
        v
    }
    pub fn get_compress(&self, v: usize) -> usize {
        if v >= self.ptrs.len() {
            return v;
        }
        let dst = self.get(v);
        self.ptrs[v].set(dst);
        dst
    }
    pub fn set(&mut self, v: usize, to: usize) {
        assert!(v <= self.ptrs.len());
        assert!(to <= self.ptrs.len());
        let root_to = self.get_compress(to);
        let root_v = self.get_compress(v);
        if root_v != root_to {
            unsafe { self.ptrs.get_unchecked(root_v) }.set(root_to);
            self.len -= 1;
        }
    }
    /// Checks if a vertex is itself the root of a tree
    pub fn is_root(&self, v: usize) -> bool {
        self.ptrs.get(v).map(|p| p.get() == v).unwrap_or(false)
    }
    pub fn compress(&mut self) {
        for i in 0..self.ptrs.len() {
            // compress it to last item always to flatten pointer chains.
            let terminal = self.get(i);
            if terminal != i {
                self.set(i, terminal);
            }
        }
    }
    pub fn extend_by(&mut self, n: usize) {
        let l = self.ptrs.len();
        for i in 0..n {
            self.ptrs.push(Cell::new(l + i));
        }
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.ptrs.len()
    }
    #[inline]
    pub fn curr_len(&self) -> usize {
        self.len
    }
}
