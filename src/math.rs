extern crate csv;

use std::{
    path::Path,
    error::Error,
    fs::File,
    ops::{Index, IndexMut},
};

use rand::{
    thread_rng,
    seq::SliceRandom,
};

/// Permute in-place all matrix rows with respect to target vector.
pub fn shuffle(X: &mut Matrix, y: &mut Vector) {
    let mut permutation: Vec<usize> = (0..X.rows()).collect();
    permutation.shuffle(&mut thread_rng());

    for indices in permutation.chunks(2) {
        let (i, j) = match indices {
            &[i, j] => (i, j),
            _ => continue
        };
        y.swap(i, j);
        for k in 0..X.cols {
            X.data.swap(i * X.cols + k, j * X.cols + k);
        }
    }
}

/// Scalar product of two slices.
pub fn dot(lhs: &[f64], rhs: &[f64]) -> f64 {
    lhs.iter().zip(rhs.iter())
        .map(|(l, r)| l * r)
        .sum()
}

// TODO: Implement.
/// Generate `f`-dependent train matrix and target with random noise.
pub fn generate<F>(n: usize, m: usize, f: F) -> (Matrix, Vector)
    where F: Fn(f64) -> f64,
{
    let X = Matrix::new(n, m);
    let y = vec![0f64; n];

    (X, y)
}

pub trait FromCSV {
    /// Read from `.csv` file.
    /// Trait is used due to type-aliasing limitations with `Vec<f64>`.
    fn read<P: AsRef<Path> + ?Sized>(filename: &P) -> Result<Self, Box<dyn Error>>
        where Self: Sized;
}

pub type Vector = Vec<f64>;

impl FromCSV for Vector {
    fn read<P: AsRef<Path> + ?Sized>(filename: &P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        let mut data = Vec::new();
        for result in reader.records() {
            let record = result?;
            data.push(record.get(0).unwrap().parse::<f64>()?);
        }

        Ok(data)
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(n: usize, m: usize) -> Self {
        Matrix {
            rows: n,
            cols: m,
            data: vec![0f64; n * m],
        }
    }

    pub fn rows(&self) -> usize { self.rows }

    pub fn cols(&self) -> usize { self.cols }

    pub fn transpose(&self) -> Self {
        let mut data = Vec::with_capacity(self.rows * self.cols);
        unsafe {
            data.set_len(self.rows * self.cols);
        }
        for i in 0..self.cols {
            for j in 0..self.rows {
                data[i * self.rows + j] = self[[j, i]];
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }
}

impl FromCSV for Matrix {
    fn read<P: AsRef<Path> + ?Sized>(filename: &P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        let mut rows = 0;
        let mut cols = 0;
        let mut data = Vec::new();
        for result in reader.records() {
            let record = result?;
            rows += 1;
            cols = record.len();
            for element in record.iter() {
                data.push(element.parse::<f64>()?);
            }
        }

        Ok(Matrix {
            rows,
            cols,
            data,
        })
    }
}

/// Return row of matrix as the iterable slice.
/// Each row represents observation, thus it's useful for regression.
impl Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.rows {
            panic!("index out of bounds: the rows num is {} but the index is {}", self.rows, idx);
        }

        &self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        let (i, j) = (idx[0], idx[1]);
        if i >= self.rows || j >= self.cols {
            panic!("index out of bounds: the dim is {},{} but the index is {},{}",
                   self.rows, self.cols, i, j);
        }

        &self.data[i * self.cols + j]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.rows {
            panic!("index out of bounds: the rows num is {} but the index is {}", self.rows, idx);
        }

        &mut self.data[idx * self.cols..(idx + 1) * self.cols]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        let (i, j) = (idx[0], idx[1]);
        if i >= self.rows || j >= self.cols {
            panic!("index out of bounds: the dim is {},{} but the index is {},{}",
                   self.rows, self.cols, i, j);
        }

        &mut self.data[i * self.cols + j]
    }
}