use crate::Pos2;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T: Default + Clone> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self(vec![vec![T::default(); cols]; rows])
    }

    pub fn same_size_as<T2>(other: &Grid<T2>) -> Self {
        Self(vec![vec![T::default(); other.cols()]; other.rows()])
    }
}

impl<T> Grid<T> {
    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0[0].len()
    }
}

impl<T> Index<Pos2<i32>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos2<i32>) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Pos2<i32>> for Grid<T> {
    fn index_mut(&mut self, index: Pos2<i32>) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

impl<T> Index<Pos2<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos2<usize>) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl<T> IndexMut<Pos2<usize>> for Grid<T> {
    fn index_mut(&mut self, index: Pos2<usize>) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}
