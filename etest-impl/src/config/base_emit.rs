use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use super::Config;

use crate::defs::*;
use crate::Function;

impl Config {
    /// Adds the `#[test]` attribute
    ///
    /// Generated attribute can be overidden by the `test_fn` configuration
    /// parameter.  When it is the literal `()`, no attribute will be
    /// generated at all.
    ///
    /// When function is `async`, it will use `#[tokio::test]`.
    ///
    /// # Examples
    ///
    /// ## skip attribute
    ///
    /// ```ignore
    /// #[etest(test_fn=())]
    /// fn test() {}
    /// ```
    ///
    /// expands to plain
    ///
    /// ```rust
    /// fn test() {}
    /// ```
    ///
    /// ## default attributes
    ///
    /// ```ignore
    /// #[etest]
    /// fn test_sync() {}
    ///
    /// #[etest]
    /// async fn test_async() {}
    /// ```
    ///
    /// expands to
    ///
    /// ```ignore
    /// #[test]
    /// fn test_sync() {}
    ///
    /// #[tokio::test]
    /// async fn test_async() {}
    /// ```
    ///
    /// ## manual attribute
    /// ```ignore
    /// #[etest(test_fn=mytest)]
    /// fn test() {}
    /// ```
    ///
    /// expands to
    ///
    /// ```ignore
    /// #[mytest]
    /// fn test() {}
    /// ```
    pub fn emit_test_decl(&self, func: &Function) -> TokenStream {
        if !self.has_test_fn() {
            return TokenStream::new();
        }

        let test_attr = match &self.test_fn {
            Some(f)	=> f.clone(),
            None if func.is_async	=> [
                TokenTree::Ident(Ident::new("tokio", Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("test", Span::mixed_site()))
            ].into_iter().collect(),
            None			=> [
                TokenTree::Ident(Ident::new("test", Span::mixed_site()))
            ].into_iter().collect(),
        };

        let mut res = vec![
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Bracket, test_attr))
        ];

        for attr in &func.attr {
            res.push(TokenTree::Punct(Punct::new('#', Spacing::Alone)));
            res.push(attr.clone());
        }

        res.into_iter().collect()
    }

    /// Adds some generic code in front of generated function; e.g.
    ///
    /// ```ignore
    /// # use etest::prelude::*;
    /// let etest_current_test = etest::ResourceUser::new();
    /// ```
    pub fn emit_generic(&self, _func: &Function) -> TokenStream {
        [
            // 'use etest::prelude::*;'
            TokenTree::Ident(Ident::new("use", Span::mixed_site())),
            TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("prelude", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Punct(Punct::new('*', Spacing::Alone)),
            TokenTree::Punct(Punct::new(';', Spacing::Alone)),

            // 'let etest_current_test = (|| { etest::ResourceUser::new() })();'
            //
            // Use the closure to interrupt propagation of #[track_caller] and
            // really the return the current location. See
            // https://rust-lang.github.io/rfcs/2091-inline-semantic.html#propagation-of-tracker
            TokenTree::Ident(Ident::new("let", Span::mixed_site())),
            TokenTree::Ident(Ident::new(VARNAME_CURENT_TEST, Span::mixed_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, [
                TokenTree::Punct(Punct::new('|', Spacing::Alone)),
                TokenTree::Punct(Punct::new('|', Spacing::Alone)),
                TokenTree::Group(Group::new(Delimiter::Brace, [
                    TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("Location", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("new", Span::mixed_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                ].into_iter().collect())),
            ].into_iter().collect())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        ].into_iter().collect()
    }

    /// Adds a check whether test shall be skipped.
    ///
    /// Used configuration parameters:
    ///
    /// - `skip_fn`: expression which is called to check whether test shall
    ///   be skipped
    ///
    /// - `skip_result`: allows to explicitly state the return value of the
    ///   test when test is skipped.  When not given, it will
    ///   `DefaultReturn::default_return()` or `()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[etest(skip_fn=check_func, skip_result=23)]
    /// fn test() -> u32 { /* ... */ }
    /// ```
    ///
    /// expands to
    ///
    /// ```ignore
    /// fn test() {
    ///     if check_func() {} else { return 23 }
    ///     /* .... */
    /// }
    /// ```
    pub fn emit_skip_fn(&self, func: &Function) -> TokenStream {
        let Some(skip_fn) = &self.skip_fn else {
            return TokenStream::new();
        };

        // inner block

        let mut inner_block = vec![
            TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("mark_skipped", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, [
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Ident(Ident::new(VARNAME_CURENT_TEST, Span::mixed_site()))
            ].into_iter().collect())),
            TokenTree::Punct(Punct::new(';', Spacing::Alone)),

            TokenTree::Ident(Ident::new("return", Span::call_site())),
        ];

        match &self.skip_result {
            Some(r)	=> inner_block.extend(r.clone()),
            None	=> inner_block.extend(func.default_return()),
        }

        // final, outer block
        [
            TokenTree::Ident(Ident::new("if", Span::mixed_site())),
            TokenTree::Punct(Punct::new('!',  Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, skip_fn.clone())),
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new())),
            TokenTree::Ident(Ident::new("else", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Brace, inner_block.into_iter().collect())),
        ].into_iter().collect()
    }

    pub fn emit_lock(&self, _func: &Function) -> TokenStream {
        //println!("uses={:?}", self.uses);
        //println!("consumes={:?}", self.consumes);

        if self.uses.is_empty() && self.consumes.is_empty() {
            return TokenStream::new();
        }

        let mut builder = vec![
            TokenTree::Ident(Ident::new("let", Span::mixed_site())),
            TokenTree::Ident(Ident::new("_resource_lock", Span::mixed_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),

            TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("ResourceBuilder", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("new", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ];

        for u in self.uses.iter() {
            builder.extend([
                TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                TokenTree::Ident(Ident::new("uses", Span::mixed_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, u.clone()))
            ]);
        }

        for c in self.consumes.iter() {
            builder.extend([
                TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                TokenTree::Ident(Ident::new("consumes", Span::mixed_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, c.clone()))
            ]);
        }

        builder.extend([
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("reserve", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, [
                TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("RESOURCES", Span::mixed_site())),
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Ident(Ident::new(VARNAME_CURENT_TEST, Span::mixed_site())),
            ].into_iter().collect())),

            TokenTree::Punct(Punct::new(';', Spacing::Joint)),
        ]);

        builder.into_iter().collect()
    }

    pub fn emit_timeout(self, func: &Function) -> TokenStream {
        let Some(timeout) = self.timeout else {
            return func.body.clone();
        };

        // std::time::Duration::from_millis(&etest_current_test, ..., move || ...)
        let mut args = vec![
            TokenTree::Punct(Punct::new('&', Spacing::Joint)),
            TokenTree::Ident(Ident::new(VARNAME_CURENT_TEST, Span::mixed_site())),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        ];

        args.extend(timeout);

        args.extend([
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        ]);

        args.extend([
            TokenTree::Ident(Ident::new("move", Span::mixed_site())),
            TokenTree::Punct(Punct::new('|', Spacing::Alone)),
            TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        ]);

        // TODO: this depends on #![feature(async_closure)]
        // https://github.com/rust-lang/rust/issues/62290
        if func.is_async {
            args.push(TokenTree::Ident(Ident::new("async", Span::mixed_site())));
        }

        args.extend(func.body.clone());

        let mut res = vec![
            TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("panic_after", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, args.into_iter().collect())),
        ];

        if func.is_async {
            res.extend([
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                TokenTree::Ident(Ident::new("await", Span::mixed_site())),
            ]);
        }

        res.into_iter().collect()
    }
}
