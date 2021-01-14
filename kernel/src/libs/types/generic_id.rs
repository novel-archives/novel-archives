use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GenericId<T, R> {
    raw_id: R,
    _phantom: PhantomData<T>,
}

impl<T, R> From<R> for GenericId<T, R> {
    fn from(raw_id: R) -> GenericId<T, R> {
        GenericId {
            raw_id,
            _phantom: PhantomData,
        }
    }
}

impl<T, R> GenericId<T, R> {
    pub fn raw_id(self) -> R {
        self.raw_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(32=>32)]
    #[test_case(55=>55)]
    #[test_case("hoge"=>"hoge")]
    fn from_into_works<T>(raw_id: T) -> T {
        struct Tag;
        let id = GenericId::<Tag, T>::from(raw_id);
        id.raw_id()
    }
}
