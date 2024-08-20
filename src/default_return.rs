/// Allows to construct the return value when test is skipped
///
/// When not explicitly given by the `skip_result` parameter, the test will
/// return [`DefaultReturn::default_return()`].
///
/// Application might need to implement an own implementation for uncommon
/// return type.
pub trait DefaultReturn {
    fn default_return() -> Self;
}

impl DefaultReturn for () {
    fn default_return() -> Self {
        Default::default()
    }
}

impl DefaultReturn for std::process::ExitCode {
    fn default_return() -> Self {
        std::process::ExitCode::SUCCESS
    }
}

impl <T: Default, E> DefaultReturn for std::result::Result<T, E> {
    fn default_return() -> Self {
        Ok(T::default())
    }
}
