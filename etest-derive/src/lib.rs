use proc_macro::TokenStream;

/// Function attribute to declare an "etest"
///
/// Supported parameters:
///
/// - `skip=<expr>`: when expression evaluates to `true`, test is skipped
///
/// - `skip_result=<expr>`: allows to set explicitly a return value when test is skipped
///
///
/// - `uses=<literal>` or `uses=[<expr>, ...]`: specifies resources which are "used" (shared)
///
/// - `consumes=<literal>` or `consumes=[<expr>, ...]`: specifies resources
///   which are "consumed" (exclusively)
///
/// - `no_default_uses`: removes basic resources from the "uses" list
///
/// - `notparallel`: consumes basic resources so that test is not run with other ones
///
///
/// - `timeout=<expr>`: test panics after the given time when not finished
///
/// See etest crate documentation for details.
#[proc_macro_attribute]
pub fn etest(attr: TokenStream, item: TokenStream) -> TokenStream {
    etest_impl::etest(attr, item)
}
