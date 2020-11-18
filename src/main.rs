#![allow(non_snake_case)]

use crate::{
    math::{FromCSV, Matrix, Vector},
    regressor::{
        lib::*,
        adam::Adam,
    },
};
use std::error::Error;

pub mod regressor;
pub mod math;

#[cfg(test)]
pub mod test;

fn main() -> Result<(), Box<dyn Error>> {
    let X = Matrix::read("./data/train/X.csv")?;
    let y = Vector::read("./data/train/y.csv")?;

    let regression = Adam::default()
        .verbose(true)
        .iterations(50000)
        .fit(X, y);

    let X = Matrix::read("./data/test/X.csv")?;
    let y = Vector::read("./data/test/y.csv")?;

    println!("Error: {:.05}", regression.mse(&X, &y));
    println!("R2 Score: {:.05}", regression.score(&X, &y));

    Ok(())
}
