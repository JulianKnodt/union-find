mod atomic;
pub use atomic::UnionFind as AtomicUnionFind;

use core::cell::Cell;
use core::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionFind<T: Copy + Eq = usize> {
    ptrs: Vec<Cell<T>>,

    len: usize,
}

pub trait UnionFindOp {
    fn find(&self, v: usize) -> usize;
    fn union(&mut self, v: usize, to: usize);
    fn is_root(&self, v: usize) -> bool {
        self.find(v) == v
    }
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
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

#[cfg(feature = "unchecked")]
macro_rules! idx {
    ($s: expr, $vi: expr) => {
        unsafe { $s.get_unchecked($vi) }
    };
}

#[cfg(not(feature = "unchecked"))]
macro_rules! idx {
    ($s: expr, $vi: expr) => {{ &$s[$vi] }};
}

/// A subset of another UnionFind. Note that all values passed should use values starting from
/// 0, not those values from the original.
#[derive(Debug, PartialEq, Eq)]
pub struct BorrowedUnionFind<'a, T: Copy + Eq = usize> {
    /// Slice of cells of original union find
    ptrs: &'a mut [Cell<T>],
    /// mutable reference to original len
    len: &'a mut usize,

    own_len: usize,
    /// The range within the original UnionFind
    r: Range<usize>,
}

impl<T: Copy + Eq> BorrowedUnionFind<'_, T> {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.ptrs.len()
    }
    #[inline]
    pub fn curr_len(&self) -> usize {
        self.own_len
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
        while let n = idx!(self.ptrs, v).get()
            && n != v
        {
            v = n;
        }
        v
    }
    pub fn get_compress(&self, v: usize) -> usize {
        let dst = self.get(v);
        idx!(self.ptrs, v).set(dst);
        dst
    }
    pub fn set(&mut self, v: usize, to: usize) {
        assert!(v <= self.ptrs.len());
        assert!(to <= self.ptrs.len());
        let root_to = self.get_compress(to);
        let root_v = self.get_compress(v);
        if root_v != root_to {
            idx!(self.ptrs, root_v).set(root_to);
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
        self.len += n;
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
        while let n = idx!(self.ptrs, v).get() as usize
            && n != v
        {
            v = n;
        }
        v
    }
    pub fn get_compress(&self, v: usize) -> usize {
        let dst = self.get(v);
        idx!(self.ptrs, v).set(dst as u32);
        dst
    }
    pub fn set(&mut self, v: usize, to: usize) {
        debug_assert!(v <= self.ptrs.len());
        debug_assert!(to <= self.ptrs.len());
        let root_to = self.get_compress(to);
        let root_v = self.get_compress(v);
        if root_v == root_to {
            return;
        }
        idx!(self.ptrs, root_v).set(root_to as u32);
        self.len -= 1;
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
        assert!(
            (l + n) < u32::MAX as usize,
            "UnionFind<u32> Will overflow with {}",
            l + n
        );
        for i in 0..n {
            let s = (l + i) as u32;
            self.ptrs.push(Cell::new(s));
        }
        self.len += n;
    }
    /// Extract a subset of this union-find, assuming that it only maps within this range to
    /// itself.
    pub fn subset_clone(&self, r: Range<usize>) -> Self {
        let offset = r.start;
        let len = r.end - offset;
        let ptrs = vec![Cell::new(0); len];
        let mut len = 0;
        for (new_i, old_i) in r.clone().enumerate() {
            let prev_v = self.ptrs[old_i].get();
            len += (prev_v as usize == old_i) as usize;
            assert!(r.contains(&(prev_v as usize)));
            ptrs[new_i].set(prev_v - offset as u32);
        }
        Self { ptrs, len }
    }

    pub fn subset<'a>(&'a mut self, r: Range<usize>) -> BorrowedUnionFind<'a, u32> {
        let own_len = self
            .ptrs
            .iter()
            .enumerate()
            .take(r.end)
            .skip(r.start)
            .filter(|(i, v)| v.get() as usize == *i)
            .count();
        let ptrs = &mut self.ptrs[r.clone()];
        let len = &mut self.len;
        BorrowedUnionFind {
            ptrs,
            len,
            own_len,
            r,
        }
    }
}

impl BorrowedUnionFind<'_, u32> {
    #[inline]
    pub fn get(&self, mut v: usize) -> usize {
        debug_assert!(self.r.contains(&(v + self.r.start)), "{v:?} {:?}", self.r);
        while let n = idx!(self.ptrs, v).get() as usize - self.r.start
            && n != v
        {
            v = n;
        }
        v
    }
    pub fn get_compress(&self, v: usize) -> usize {
        let dst = self.get(v);
        unsafe { self.ptrs.get_unchecked(v) }.set((dst + self.r.start) as u32);
        dst
    }
    pub fn set(&mut self, v: usize, to: usize) {
        debug_assert!(self.r.contains(&(v + self.r.start)));
        debug_assert!(self.r.contains(&(to + self.r.start)));

        let root_to = self.get_compress(to);
        let root_v = self.get_compress(v);
        if root_v == root_to {
            return;
        }
        idx!(self.ptrs, root_v).set((root_to + self.r.start) as u32);
        *self.len -= 1;
        self.own_len -= 1;
    }
    /// Checks if a vertex is itself the root of a tree
    pub fn is_root(&self, v: usize) -> bool {
        self.ptrs
            .get(v)
            .map(|p| p.get() as usize == v)
            .unwrap_or(false)
    }
}

macro_rules! impl_basic {
    ($t: ty) => {
        impl UnionFindOp for $t {
            #[inline]
            fn find(&self, v: usize) -> usize {
                self.get_compress(v)
            }
            #[inline]
            fn union(&mut self, v: usize, to: usize) {
                self.set(v, to)
            }
            #[inline]
            fn len(&self) -> usize {
                self.len
            }
            #[inline]
            fn capacity(&self) -> usize {
                self.ptrs.len()
            }
        }
    };
    (BORROWED $t: ty) => {
        impl UnionFindOp for $t {
            #[inline]
            fn find(&self, v: usize) -> usize {
                self.get_compress(v)
            }
            #[inline]
            fn union(&mut self, v: usize, to: usize) {
                self.set(v, to)
            }
            #[inline]
            fn len(&self) -> usize {
                self.own_len
            }
            #[inline]
            fn capacity(&self) -> usize {
                self.ptrs.len()
            }
        }
    };
}

impl_basic!(UnionFind<usize>);
impl_basic!(UnionFind<u32>);
impl_basic!(BORROWED BorrowedUnionFind<'_, u32>);

#[test]
fn test_subset_clone() {
    let mut v = UnionFind::new_u32(32);
    let s = v.subset_clone(16..32);
    assert_eq!(s.curr_len(), 16);
    assert_eq!(s.capacity(), 16);

    v.set(18, 19);
    let s = v.subset_clone(16..32);
    assert_eq!(s.curr_len(), 15);
    assert_eq!(s.capacity(), 16);
    assert!(!s.is_root(2));
    assert_eq!(s.get(2), 3);
}

#[test]
fn test_subset() {
    let mut v = UnionFind::new_u32(32);
    let mut s = v.subset(16..32);
    assert_eq!(s.curr_len(), 16);
    assert_eq!(s.capacity(), 16);
    assert_eq!(s.curr_len(), 16);

    assert_eq!(s.get(1), 1);
    assert_eq!(s.get_compress(1), 1);
    s.set(1, 2);
    assert_eq!(s.curr_len(), 15);
    assert_eq!(s.get(1), 2);

    assert_eq!(v.curr_len(), 31);

    v.set(20, 21);
    let s = v.subset(16..32);
    assert_eq!(s.curr_len(), 14);
    assert_eq!(s.capacity(), 16);
    assert!(!s.is_root(4));
    assert_eq!(s.get(4), 5);
}
