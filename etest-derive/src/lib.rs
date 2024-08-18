use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn etest(attr: TokenStream, item: TokenStream) -> TokenStream {
    etest_impl::etest(attr, item)
}
