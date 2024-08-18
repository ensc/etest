use proc_macro::{Delimiter, Group, Span, TokenStream, TokenTree};

use crate::{ Config, Function };
use crate::utils::err;

pub fn etest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cfg = match Config::parse(attr) {
        Ok(c)	=> c,
        Err(e)	=> return e,
    };

    let func = match Function::parse(item) {
        Err(e)	=> return err(Span::call_site(), &format!("failed to parse function {e:?}")),
        Ok(f)	=> f,
    };

    let mut res = Vec::new();

    // this adds '#[test]' or so
    res.extend(cfg.emit_test_decl(&func));

    let mut body = Vec::<TokenTree>::new();

    body.extend(cfg.emit_generic(&func));

    body.extend(cfg.emit_skip_fn(&func));
    body.extend(cfg.emit_lock(&func));
    body.extend(cfg.emit_timeout(&func));

    res.extend(func.decl);
    res.push(TokenTree::Group(Group::new(Delimiter::Brace, body.into_iter().collect())));

    res.into_iter().collect()
}
