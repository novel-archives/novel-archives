use general::prelude::*;
use harsh::Harsh;
use std::convert::TryFrom;
use std::sync::Mutex;
pub struct IdGenerator {
    harsh: Harsh,
    prefix_key: Mutex<u64>,
}

impl IdGenerator {
    const PROJECT_EPOCH: i64 = 1610722800000000000;
    pub fn new<T: Into<Vec<u8>>>(salt: T) -> IdGenerator {
        IdGenerator {
            harsh: Harsh::builder().salt(salt).build().unwrap(),
            prefix_key: Mutex::new(rand::random::<u32>() as u64),
        }
    }
    pub fn generate<T>(&self) -> Result<Id<T>, Error> {
        let elapsed = times::now().timestamp_nanos() - Self::PROJECT_EPOCH;
        let mut prefix_key_mutex = self.prefix_key.lock().unwrap();
        let prefix_key = *prefix_key_mutex;
        if prefix_key == u64::MAX {
            *prefix_key_mutex = 0;
        }
        *prefix_key_mutex += 1;
        let h = self.harsh.encode(&[prefix_key, elapsed as u64]);
        Ok(Id::try_from(h).unwrap())
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
        let id_gen = IdGenerator::new("salt");
        let id = id_gen.generate::<IdTag>().unwrap();
        assert_ne!(id.raw_id(), "");
    }

    #[test]
    fn prefix_key_works() {
        let id_gen = IdGenerator::new("salt");
        let before = *id_gen.prefix_key.lock().unwrap();
        let _ = id_gen.generate::<IdTag>();
        assert_eq!(before + 1, *id_gen.prefix_key.lock().unwrap());
    }
}
