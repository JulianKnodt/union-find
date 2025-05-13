use super::UnionFindOp;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

#[derive(Debug)]
pub struct UnionFind {
    ptrs: Vec<AtomicU32>,
    len: usize,
}

impl UnionFind {
    #[inline]
    pub fn new(len: usize) -> Self {
        assert!(len < u32::MAX as usize, "UnionFind<u32> will overflow");
        let ptrs = (0..len).map(|_| AtomicU32::new(0)).collect::<Vec<_>>();
        for (i, ptr) in ptrs.iter().enumerate() {
            ptr.store(i as u32, Ordering::SeqCst);
        }
        Self { ptrs, len }
    }
    #[inline]
    pub fn get(&self, v: usize) -> usize {
        let mut v = v as u32;
        while let n = unsafe { self.ptrs.get_unchecked(v as usize) }.load(Ordering::SeqCst)
            && n != v
        {
            v = n;
        }
        v as usize
    }
    pub fn get_compress(&self, v: usize) -> usize {
        let dst = self.get(v);
        unsafe { self.ptrs.get_unchecked(v) }.store(dst as u32, Ordering::SeqCst);
        dst
    }
    // safe since this union find is exclusively held, and cannot be updated in parallel.
    pub fn set(&mut self, v: usize, to: usize) {
        debug_assert!(v <= self.ptrs.len());
        debug_assert!(to <= self.ptrs.len());
        let root_to = self.get_compress(to);
        let root_v = self.get_compress(v);
        if root_v != root_to {
            unsafe { self.ptrs.get_unchecked(root_v) }.store(root_to as u32, Ordering::SeqCst);
            self.len -= 1;
        }
    }
}

impl UnionFindOp for UnionFind {
    fn find(&self, v: usize) -> usize {
        self.get_compress(v)
    }
    fn union(&mut self, v: usize, to: usize) {
        self.set(v, to);
    }
    fn len(&self) -> usize {
        self.len
    }
    fn capacity(&self) -> usize {
        self.ptrs.len()
    }
}
