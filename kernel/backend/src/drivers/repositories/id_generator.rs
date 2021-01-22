use crate::prelude::*;
pub struct IdGenerator {}

impl IdGenerator {
    pub fn new() -> Self {
        Self {}
    }
    pub fn generate<T>(&self) -> Result<Id<T>, Error> {
        Ok(Id::try_new(xid::new().to_string()).unwrap())
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum Error {}

#[cfg(test)]
mod tests {
    use super::*;

    struct IdTag;
    #[test]
    fn generate_works() {
        let id_gen = IdGenerator::new();
        let id = id_gen.generate::<IdTag>().unwrap();
        assert_ne!(id.raw_id(), "");
    }
}
