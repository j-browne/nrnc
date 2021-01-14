use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReaclibError {
    #[error("invalid chapter ({0})")]
    InvalidChapter(u8),
    #[error("error parsing int")]
    IntParse(#[from] std::num::ParseIntError),
}

pub type Res<T> = Result<T, ReaclibError>;