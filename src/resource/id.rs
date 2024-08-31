use std::borrow::Cow;

/// A resource which can be "used" or "consumed"
///
/// To be used with the `uses` and `consumes` parameters of `#[etest]`.  For type
/// safety custom types can be specified which implement `Into<ResourceId>`.
///
/// E.g.
///
/// ```
/// # use etest::ResourceId;
/// # fn is_hdmi_connected() -> bool { true }
/// enum Output {
///     Auto,
///     Hdmi,
///     Lvds,
/// }
///
/// impl From<Output> for ResourceId
/// {
///     fn from(o: Output) -> Self {
///         let o = match o {
///             // this is evaluated at runtime of the test
///             Output::Auto if is_hdmi_connected() => Output::Hdmi,
///             Output::Auto                        => Output::Lvds,
///             o => o,
///         };
///
///         match o {
///             Output::Hdmi => "hdmi".into(),
///             Output::Lvds => "lvds".into(),
///             Output::Auto => unreachable!(),
///         }
///     }
/// }
///
/// # use etest::etest;
/// #[etest(consumes=[Output])]
/// fn test() {}
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ResourceIdImpl<'a> {
    /// Normal resource
    Id(Cow<'a, str>),
}

pub type ResourceId = ResourceIdImpl<'static>;

impl <'a> ResourceIdImpl<'a> {
    pub const fn new(id: &'a str) -> Self {
        Self::Id(Cow::Borrowed(id))
    }

    pub fn from_string(s: String) -> Self {
        Self::Id(Cow::Owned(s))
    }
}

impl <'a> From<&'a str> for ResourceIdImpl<'a> {
    fn from(val: &'a str) -> Self {
        Self::new(val)
    }
}

impl From<String> for ResourceIdImpl<'_> {
    fn from(val: String) -> Self {
        Self::from_string(val)
    }
}
