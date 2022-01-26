
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ValueError(String),
}

#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn io_error<T>(e: std::io::Error) -> Result<T> {
    Err(Error::IoError(e))
}