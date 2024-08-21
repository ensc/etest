use std::time::Duration;

/// Wrapper around [`std::time::Duration`].
///
/// Parameter of `timeout` excepts some value implements
/// [`Into<Timeout>`](Timeout).
#[derive(Clone, Copy, Debug)]
pub struct Timeout(Duration);

impl Timeout {
    pub fn new(d: Duration) -> Self {
        Self(d)
    }

    pub fn duration(self) -> Duration {
        self.0
    }
}

/// Converts a milliseconds value in a [`Timeout`]
impl From<i32> for Timeout {
    fn from(value: i32) -> Self {
        assert!(value >= 0);
        Self(Duration::from_millis(value as u64))
    }
}

/// Converts a milliseconds value in a [`Timeout`]
impl From<u32> for Timeout {
    fn from(value: u32) -> Self {
        Self(Duration::from_millis(value as u64))
    }
}

/// Converts a milliseconds value in a [`Timeout`]
impl From<u64> for Timeout {
    fn from(value: u64) -> Self {
        Self(Duration::from_millis(value))
    }
}

impl From<Duration> for Timeout {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}
