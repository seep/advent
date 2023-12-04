use itertools::Itertools;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Array2D<T> {
    values: Vec<T>,
    rows: usize,
    cols: usize,
}

pub type Array2DIndex = (usize, usize);

impl<T> Array2D<T> {
    /// Creates a new Array2D filled with the default value.
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default,
        T: Clone,
    {
        Array2D {
            values: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    /// Creates a new Array2D filled with the given [value].
    pub fn fill(value: T, rows: usize, cols: usize) -> Self
    where
        T: Clone,
    {
        Array2D {
            values: vec![value; rows * cols],
            rows,
            cols,
        }
    }

    /// Creates a new Array2D from a slice of values in row-major order.
    pub fn from_slice(values: &[T], rows: usize, cols: usize) -> Self
    where
        T: Clone,
    {
        assert_eq!(values.len(), rows * cols);
        Array2D {
            values: values.to_vec(),
            rows,
            cols,
        }
    }

    /// Returns the number of rows in the array.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the array.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the flat index within the values vector of the specified [row] and [col], or None if out of bounds.
    pub fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(self.cols * row + col)
        } else {
            None
        }
    }

    /// Returns a reference to the element at the specified [row] and [col], or None if out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let index = self.get_index(row, col)?;
        self.values.get(index)
    }

    /// Returns a mutable reference to the element at the specified [row] and [col], or None if out of bounds.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let index = self.get_index(row, col)?;
        self.values.get_mut(index)
    }

    /// Returns a reference to the element at the specified [row] and [col], or None if out of bounds.
    pub fn get_tuple(&self, (row, col): Array2DIndex) -> Option<&T> {
        self.get(row, col)
    }

    /// Returns a mutable reference to the element at the specified [row] and [col], or None if out of bounds.
    pub fn get_mut_tuple(&mut self, (row, col): Array2DIndex) -> Option<&mut T> {
        self.get_mut(row, col)
    }

    /// Set the [value] at the specified [row] and [col].
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col).expect("index out of bounds");
        self.values[index] = value;
    }

    /// Returns an iterator (in row major order) over the array.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    /// Returns an iterator (in row major order) that allows modifying each value.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    /// Returns an iterator over the 2D indices of the array.
    pub fn iter_indices(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.rows).cartesian_product(0..self.cols)
    }

    /// Returns an iterator over the values in the specified row.
    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        assert!(row < self.rows);
        self.values.iter().skip(row * self.cols).take(self.cols)
    }

    /// Returns an iterator over the values in the specified column.
    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = &T> {
        assert!(col < self.cols);
        self.values.iter().skip(col).step_by(self.cols)
    }

    /// Returns an iterator over each row in the array.
    /// The iterator yields an inner iterator over the values in each row.
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows).map(|r| self.iter_row(r))
    }

    /// Returns an iterator over each column in the array.
    /// The iterator yields an inner iterator over the values in each column.
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.cols).map(|c| self.iter_col(c))
    }

    /// Returns an iterator over each element adjacent to the specified [row] and [col].
    pub fn iter_adjacent(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
        MooreNeighborhoodIterator::new((row, col), false).flat_map(|i| self.get_tuple(i))
    }

    /// Returns an iterator that enumerates each value in the array with its 2D index.
    pub fn enumerate(&self) -> impl Iterator<Item = (Array2DIndex, &T)> {
        self.iter_indices().zip(self.iter())
    }

    /// Returns an iterator that enumerates each value in the array with its 2D index, that allows modifying each value.
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Array2DIndex, &mut T)> {
        self.iter_indices().zip(self.iter_mut())
    }
}

impl<T> Index<Array2DIndex> for Array2D<T> {
    type Output = T;

    fn index(&self, (row, col): Array2DIndex) -> &Self::Output {
        self.get(row, col).expect("index out of bounds")
    }
}

impl<T> IndexMut<Array2DIndex> for Array2D<T> {
    fn index_mut(&mut self, (row, col): Array2DIndex) -> &mut Self::Output {
        self.get_mut(row, col).expect("index out of bounds")
    }
}

/// Data structure for iterating over the Moore neighborhood of a 2D index.
struct MooreNeighborhoodIterator {
    cells: [Option<Array2DIndex>; 9],
    index: usize,
}

impl MooreNeighborhoodIterator {
    pub fn new(center: Array2DIndex, include_center: bool) -> MooreNeighborhoodIterator {
        let mut cells = [None; 9];

        cells[0] = moore_neighbor(center, 0);
        cells[1] = moore_neighbor(center, 1);
        cells[2] = moore_neighbor(center, 2);
        cells[3] = moore_neighbor(center, 3);
        cells[4] = moore_neighbor(center, 4);
        cells[5] = moore_neighbor(center, 5);
        cells[6] = moore_neighbor(center, 6);
        cells[7] = moore_neighbor(center, 7);
        cells[8] = moore_neighbor(center, 8);

        if !include_center {
            cells[4] = None;
        }

        Self { cells, index: 0 }
    }
}

impl Iterator for MooreNeighborhoodIterator {
    type Item = Array2DIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.index;

        while next < self.cells.len() && self.cells[next].is_none() {
            next += 1;
        }

        self.index = next + 1;

        if next < self.cells.len() {
            return self.cells[next];
        }

        None
    }
}

/// Returns the Moore neighbor relative to the specified index, or None if the the coordinate would overflow.
#[allow(clippy::manual_range_patterns)]
fn moore_neighbor((row, col): Array2DIndex, moore: usize) -> Option<Array2DIndex> {
    Option::zip(
        match moore {
            0 | 1 | 2 => row.checked_sub(1),
            3 | 4 | 5 => Some(row),
            6 | 7 | 8 => row.checked_add(1),
            _ => panic!(),
        },
        match moore {
            0 | 3 | 6 => col.checked_sub(1),
            1 | 4 | 7 => Some(col),
            2 | 5 | 8 => col.checked_add(1),
            _ => panic!(),
        },
    )
}
