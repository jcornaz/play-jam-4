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

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.cells.get(self.index_of(coord)?)
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let index = self.index_of(coord)?;
        self.cells.get_mut(index)
    }

    pub fn set(&mut self, coord: Coord, mut cell: T) -> Option<T> {
        core::mem::swap(self.get_mut(coord)?, &mut cell);
        Some(cell)
    }

    pub fn index_of(&self, Coord { x, y }: Coord) -> Option<usize> {
        let x: usize = x.try_into().ok().filter(|x| *x < self.width)?;
        let y: usize = y.try_into().ok().filter(|y| *y < self.height)?;
        Some(self.width * y + x)
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<[i32; 2]> for Coord {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<Coord> for [i32; 2] {
    fn from(Coord { x, y }: Coord) -> Self {
        [x, y]
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Coord> for (i32, i32) {
    fn from(Coord { x, y }: Coord) -> Self {
        (x, y)
    }
}
