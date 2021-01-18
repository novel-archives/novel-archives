use std::marker::PhantomData;

pub type RawId = String;
pub struct Id<T> {
    raw_id: RawId,
    _phantom: PhantomData<T>,
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, r: &Self) -> bool {
        self.raw_id == r.raw_id
    }
}

impl<T> Id<T> {
    pub fn try_new(raw_id: RawId) -> Result<Self, Error> {
        if raw_id.is_empty() {
            Err(Error::Empty)
        } else {
            Ok(Self {
                raw_id,
                _phantom: PhantomData,
            })
        }
    }
    pub fn raw_id(&self) -> &RawId {
        &self.raw_id
    }
}

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("IDが空文字列です")]
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge".to_string()=>Ok("hoge".to_string()))]
    #[test_case("".to_string()=>Err(Error::Empty))]
    fn from_into_works(raw_id: RawId) -> Result<RawId, Error> {
        struct Tag;
        let id = Id::<Tag>::try_new(raw_id)?;
        Ok(id.raw_id().clone())
    }

    #[test_case("hoge".to_string(),"fuga".to_string()=>Ok(false))]
    fn eq_works(a: RawId, b: RawId) -> Result<bool, Error> {
        struct Tag;
        let ida = Id::<Tag>::try_new(a)?;
        let idb = Id::<Tag>::try_new(b)?;
        Ok(ida == idb)
    }
}
