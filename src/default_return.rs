pub trait DefaultReturn {
    fn default_return() -> Self;
}

impl DefaultReturn for () {
    fn default_return() -> Self {
        Default::default()
    }
}

impl <T: Default, E> DefaultReturn for std::result::Result<T, E> {
    fn default_return() -> Self {
        Ok(T::default())
    }
}
