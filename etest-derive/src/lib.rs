use proc_macro::TokenStream;

/// Function attribute to declare an "etest"
///
/// See etest crate documentation for details.
#[proc_macro_attribute]
pub fn etest(attr: TokenStream, item: TokenStream) -> TokenStream {
    etest_impl::etest(attr, item)
}
