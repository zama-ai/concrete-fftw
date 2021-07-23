use crate::array::Alignment;
use std::fmt::{Display, Formatter};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidPlanError {},
    InputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },
    OutputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidPlanError {} => write!(f, "Invalid Plan"),
            Error::OutputArrayMismatch { expect, actual } => {
                write!(
                    f,
                    "Output array mismatch: expect={:?}, actual={:?}",
                    expect, actual
                )
            }
            Error::InputArrayMismatch { expect, actual } => {
                write!(
                    f,
                    "Input array mismatch: expect={:?}, actual={:?}",
                    expect, actual
                )
            }
        }
    }
}

impl std::error::Error for Error {}
