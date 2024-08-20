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
pub struct ResourceId {
    id:		String,
}

impl ResourceId {
    pub fn new(id: &str) -> Self {
        Self {
            id:	id.to_string()
        }
    }
}

impl From<&str> for ResourceId {
    fn from(val: &str) -> Self {
        ResourceId {
            id:	val.to_string()
        }
    }
}
