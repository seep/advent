use itertools::Itertools;

#[derive(Clone)]
pub struct Array2D<T> {
    values: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Array2D<T> {
    /// Creates a new Array2D filled with the default value.
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default,
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

    /// Get the value at the specified [row] and [col].
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.values[self.cols * row + col]
    }

    /// Set the [value] at the specified [row] and [col].
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.values[self.cols * row + col] = value;
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

    /// Returns an iterator that enumerates each value in the array with its 2D index.
    pub fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.iter_indices().zip(self.iter())
    }

    /// Returns an iterator that enumerates each value in the array with its 2D index, that allows modifying each value.
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.iter_indices().zip(self.iter_mut())
    }
}
