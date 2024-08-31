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

    /// An empty resource which will be ignored.
    ///
    /// Used e.g. when generating a list of resources dynamically where some
    /// of the resources are optional and can be omitted.
    None,

    /// Basic resource which is used by all tests.
    ///
    /// Unless specified else (by the `no_default_uses` attribute), this
    /// resource will be implicitly added to the "uses" list of every test.
    /// To avoid parallel execution with other ones, a test can add this
    /// resource type to its "consumes" list by the `notparallel` attribute.
    Basic,
}

pub type ResourceId = ResourceIdImpl<'static>;

impl <'a> ResourceIdImpl<'a> {
    pub const fn new(id: &'a str) -> Self {
        Self::Id(Cow::Borrowed(id))
    }

    pub fn from_string(s: String) -> Self {
        Self::Id(Cow::Owned(s))
    }

    pub fn is_some(&self) -> bool {
        self != &Self::None
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

/// Maps an option to a [`ResourceId`].
///
/// Value of `None` maps to the special [`ResourceId::None`] resource type
/// which will be ignored when building list of resources dynamically.
impl <'a, T> From<Option<T>> for ResourceIdImpl<'a>
where
    T: Into<ResourceIdImpl<'a>>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            None	=> Self::None,
            Some(v)	=> v.into(),
        }
    }
}
