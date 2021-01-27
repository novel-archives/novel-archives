use super::*;

pub struct SeriesCreationCommand {}

pub struct SeriesFactory {}

pub trait SeriesIdGenerator {
    fn generate(&self) -> result::Result<Id<Series>>;
}

impl SeriesFactory {
    pub fn create(_: SeriesCreationCommand) -> Result<Series, Error> {
        Err(Error::Unknown)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("")]
    Unknown,
}
