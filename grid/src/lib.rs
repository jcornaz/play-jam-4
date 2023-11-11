#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

impl<T> Grid<T> {
    pub fn new_with(width: usize, height: usize, init_cell: impl Fn() -> T) -> Self {
        let mut cells = Vec::new();
        cells.resize_with(width * height, init_cell);
        Self {
            cells,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, coord: impl Into<[usize; 2]>) -> Option<&T> {
        self.cells.get(self.index_of(coord)?)
    }

    pub fn get_mut(&mut self, coord: impl Into<[usize; 2]>) -> Option<&mut T> {
        let index = self.index_of(coord)?;
        self.cells.get_mut(index)
    }

    pub fn set(&mut self, coord: impl Into<[usize; 2]>, mut cell: T) -> Option<T> {
        core::mem::swap(self.get_mut(coord)?, &mut cell);
        Some(cell)
    }

    pub fn index_of(&self, coord: impl Into<[usize; 2]>) -> Option<usize> {
        let [x, y] = coord.into();
        if x < self.width && y < self.height {
            Some(self.width * y + x)
        } else {
            None
        }
    }
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, || T::default())
    }

    pub fn from_iter(width: usize, height: usize, iter: impl IntoIterator<Item = T>) -> Self {
        let len = width * height;
        let mut cells: Vec<T> = iter.into_iter().take(len).collect();
        cells.resize_with(len, || T::default());
        Self {
            cells,
            width,
            height,
        }
    }
}
