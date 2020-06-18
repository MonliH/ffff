use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
where
    T: Add<Output = T>,
{
    pub values: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix<T>
where
    T: Add<Output = T> + std::ops::AddAssign + Default + Clone,
{
    pub fn from(vector: Vec<Vec<T>>) -> Self {
        Matrix {
            rows: vector.len(),
            cols: vector.first().unwrap_or(&vec![]).len(),
            values: vector,
        }
    }

    pub fn alloca(rows: usize, columns: usize) -> Self {
        let cols = vec![Default::default(); columns];
        let values = vec![cols; rows];
        Matrix {
            rows,
            cols: columns,
            values,
        }
    }

    #[allow(non_snake_case)]
    pub fn T(self) -> Self {
        let mut c = Self::alloca(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                c.values[j][i] = self.values[i][j].clone();
            }
        }
        c
    }

    pub fn mapped<C: Fn(T) -> T + Sync>(self, func: &C) -> Self {
        Matrix {
            cols: self.cols,
            rows: self.rows,
            values: self
                .values
                .into_iter()
                .map(|row| row.into_iter().map(|x| func(x)).collect::<Vec<_>>())
                .collect(),
        }
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T>
        + std::ops::MulAssign
        + std::ops::Add<Output = T>
        + Default
        + Clone
        + std::fmt::Debug
        + std::ops::AddAssign,
{
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(self.cols, other.rows);
        let a = self;
        let b = other;
        let n = a.rows;
        let m = a.cols;
        let p = b.cols;

        let mut c = Self::alloca(a.cols, b.cols);

        for i in 0..n {
            for j in 0..p {
                let mut sum: T = Default::default();
                for k in 0..m {
                    sum += a.values[i][k].clone() * b.values[k][j].clone()
                }
                c.values[i][j] = sum;
            }
        }

        c
    }
}

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + std::ops::AddAssign,
{
    type Output = Matrix<T>;

    fn add(mut self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(self.values.len(), other.values.len());
        for (i, value) in other.values.into_iter().enumerate() {
            for (j, actual_val) in value.into_iter().enumerate() {
                self.values[i][j] += actual_val;
            }
        }

        self
    }
}

impl<T> Add<T> for Matrix<T>
where
    T: Add<Output = T> + std::ops::AddAssign + Clone,
{
    type Output = Matrix<T>;

    fn add(mut self, other: T) -> Matrix<T> {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.values[i][j] += other.clone();
            }
        }

        self
    }
}

impl<T> AddAssign<T> for Matrix<T>
where
    T: Add<Output = T> + std::ops::AddAssign + Clone,
{
    fn add_assign(&mut self, other: T) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.values[i][j] += other.clone();
            }
        }
    }
}

#[macro_export]
macro_rules! mat {
    ($($($e: expr),+);*) => {{
        let mut vec = Vec::new();
        $(
            let mut row = Vec::new();
            $(
                row.push($e);
             )+
            vec.push(row);
         )*

            Matrix::from(vec)
    }};
}

fn main() {
    let no = mat![10i8, 10i8, 10i8, 10i8];
    let super_complex = Matrix::alloca(4, 4).mapped(&|x| x + 2);
    let mut yes = (mat![4i8;4i8;4i8;4i8].T() + no) * super_complex;
    yes += -10;

    println!(
        "{}",
        yes.values
            .first()
            .unwrap()
            .into_iter()
            .map(|&x| x as u8 as char)
            .collect::<String>()
    );
}
