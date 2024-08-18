#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Location {
    loc:	std::panic::Location<'static>,
}

impl Location {
    #[track_caller]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            loc: *std::panic::Location::caller(),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.loc.fmt(f)?;

        if let Some(name) = std::thread::current().name() {
            " (".fmt(f)?;
            name.fmt(f)?;
            ")".fmt(f)?;
        }

        Ok(())
    }
}
