use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub fn err(span: Span, msg: &str) -> TokenStream {
    eprintln!("{:?} {msg}", span);

    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(msg))].into_iter().collect(),
        ))
    ].into_iter().collect()
}

pub fn empty_args() -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new()))
}
