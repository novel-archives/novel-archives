use crate::prelude::*;
use harsh::Harsh;
use snowflake::SnowflakeIdGenerator;
use std::sync::{Arc, Mutex};

pub struct IdGenerator {
    harsh: Harsh,
}

impl IdGenerator {
    pub fn new<T: Into<Vec<u8>>>(salt: T) -> Self {
        Self {
            harsh: Harsh::builder().salt(salt).build().unwrap(),
        }
    }
    pub fn generate<T>(&self) -> Result<Id<T>, Error> {
        let instance = Self::get_instance();
        let mut generator = instance.lock().unwrap();
        let row_number_id = generator.lazy_generate() as u64;
        let h = self.harsh.encode(&[row_number_id]);
        Ok(Id::try_new(h).unwrap())
    }
    fn get_instance() -> Arc<Mutex<SnowflakeIdGenerator>> {
        static mut SINGLETON: Option<Arc<Mutex<SnowflakeIdGenerator>>> = None;
        unsafe {
            SINGLETON
                .get_or_insert_with(|| {
                    Arc::new(Mutex::new(SnowflakeIdGenerator::new(
                        rand::random::<i32>() & 0b11111,
                        rand::random::<i32>() & 0b11111,
                    )))
                })
                .clone()
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {}

#[cfg(test)]
mod tests {
    use super::*;

    struct IdTag;
    #[test]
    fn generate_works() {
        let id_gen = IdGenerator::new("salt");
        let id = id_gen.generate::<IdTag>().unwrap();
        assert_ne!(id.raw_id(), "");
    }
}
