pub use failure::{Error, Fail};

pub type Result<T> = std::result::Result<T, Error>;

pub fn from<T>(e: T) -> Error
where
    T: Fail,
{
    Error::from(e)
}
