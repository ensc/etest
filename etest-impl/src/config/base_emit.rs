use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

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
    /// ```rust
    /// #[etest_derive::etest(test_fn=()]
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
    /// ```rust
    /// #[etest_derive::etest]
    /// fn test_sync() {}

    /// #[etest_derive::etest]
    /// async fn test_async() {}
    /// ```
    ///
    /// expands to
    ///
    /// ```rust
    /// #[test]
    /// fn test_sync() {}

    /// #[tokio::test]
    /// async fn test_async() {}
    /// ```
    ///
    /// ## manual attribute
    /// ```rust
    /// [etest_derive::etest(test_fn=mytest]
    /// fn test() {}
    /// ```
    ///
    /// expands to
    ///
    /// ```rust
    /// #[mytest]
    /// fn test() {}
    /// ```
    pub fn emit_test_decl(&self, func: &Function) -> TokenStream {
        if !self.has_test_fn() {
            return TokenStream::new();
        }

        let func = match &self.test_fn {
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

        vec![
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Bracket, func))
        ].into_iter().collect()
    }

    /// Adds some generic code in front of generated function; e.g.
    ///
    /// ```rust
    /// use etest::prelude::*;
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

            // 'let etest_current_test = etest::ResourceUser::new();'
            TokenTree::Ident(Ident::new("let", Span::mixed_site())),
            TokenTree::Ident(Ident::new(VARNAME_CURENT_TEST, Span::mixed_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("Location", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("new", Span::mixed_site())),
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
    /// ```rust
    /// [etest_derive::etest(skip_fn=check_func, skip_result=23]
    /// fn test() -> u32 { /* ... */ }
    /// ```
    ///
    /// expands to
    ///
    /// ```rust
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

        let mut res = vec![
            TokenTree::Ident(Ident::new("if", Span::mixed_site())),
            TokenTree::Punct(Punct::new('!',  Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, skip_fn.clone())),
        ];

        res.extend([
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new())),
            TokenTree::Ident(Ident::new("else", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Brace, inner_block.into_iter().collect())),
        ]);

        res.into_iter().collect()
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
        let Some(timeout) = self.timeout_ms else {
            return func.body.clone();
        };

        // std::time::Duration::from_millis(&etest_current_test, ..., move || ...)
        let mut args = vec![
            TokenTree::Punct(Punct::new('&', Spacing::Joint)),
            TokenTree::Ident(Ident::new(VARNAME_CURENT_TEST, Span::mixed_site())),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),

            TokenTree::Ident(Ident::new("std", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("time", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("Duration", Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("from_millis", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, [
                TokenTree::Literal(Literal::u64_unsuffixed(timeout))
            ].into_iter().collect())),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),

            TokenTree::Ident(Ident::new("move", Span::mixed_site())),
            TokenTree::Punct(Punct::new('|', Spacing::Alone)),
            TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        ];

        args.extend(func.body.clone());

        let res = vec![
            TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("panic_after", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, args.into_iter().collect())),
        ];

        res.into_iter().collect()
    }
}
