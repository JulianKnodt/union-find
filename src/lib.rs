#![feature(let_chains)]

use std::cell::Cell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionFind<T: Copy + Eq = usize> {
    ptrs: Vec<Cell<T>>,

    len: usize,
}

impl<T: Copy + Eq> UnionFind<T> {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.ptrs.len()
    }
    #[inline]
    pub fn curr_len(&self) -> usize {
        self.len
    }
}

impl UnionFind<usize> {
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
}

impl UnionFind<u32> {
    #[inline]
    pub fn new_u32(len: usize) -> Self {
        assert!(len < u32::MAX as usize, "UnionFind<u32> will overflow");
        let ptrs = vec![Cell::new(0); len];
        for (i, ptr) in ptrs.iter().enumerate() {
            ptr.set(i as u32);
        }
        Self { ptrs, len }
    }
    #[inline]
    pub fn get(&self, mut v: usize) -> usize {
        if v >= self.ptrs.len() {
            return v;
        }
        while let n = unsafe { self.ptrs.get_unchecked(v).get() }
            && n as usize != v
        {
            v = n as usize;
        }
        v
    }
    pub fn get_compress(&self, v: usize) -> usize {
        if v >= self.ptrs.len() {
            return v;
        }
        let dst = self.get(v);
        self.ptrs[v].set(dst as u32);
        dst
    }
    pub fn set(&mut self, v: usize, to: usize) {
        assert!(v <= self.ptrs.len());
        assert!(to <= self.ptrs.len());
        let root_to = self.get_compress(to);
        let root_v = self.get_compress(v);
        if root_v != root_to {
            unsafe { self.ptrs.get_unchecked(root_v) }.set(root_to as u32);
            self.len -= 1;
        }
    }
    /// Checks if a vertex is itself the root of a tree
    pub fn is_root(&self, v: usize) -> bool {
        self.ptrs
            .get(v)
            .map(|p| p.get() as usize == v)
            .unwrap_or(false)
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
        assert!((l + n) < u32::MAX as usize, "UnionFind<u32> Will overflow with {}", l + n);
        for i in 0..n {
            let s = (l + i) as u32;
            self.ptrs.push(Cell::new(s));
        }
    }
}
