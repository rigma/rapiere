use super::Index;
use smallvec::{smallvec, SmallVec};

#[derive(Clone, Debug)]
pub struct Register<T: PartialEq>(SmallVec<[T; 8]>);

impl<T: PartialEq> Register<T> {
    pub const MAX_SIZE: usize = 256;

    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    pub fn get(&self, index: Index) -> Option<&T> {
        self.0.get(index as usize)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        self.0.get_mut(index as usize)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, value: T) -> Result<Option<&T>, ()> {
        if self.remaining_size() == 0 || self.0.contains(&value) {
            return Err(());
        }

        self.0.push(value);
        Ok(self.0.last())
    }

    #[inline(always)]
    pub fn remaining_size(&self) -> usize {
        Self::MAX_SIZE - self.0.len()
    }

    #[inline(always)]
    pub fn remove(&mut self, index: Index) -> T {
        self.0.remove(index as usize)
    }
}

impl<T: PartialEq> Default for Register<T> {
    #[inline(always)]
    fn default() -> Self {
        Self(smallvec![])
    }
}
