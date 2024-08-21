use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::defs::*;
use crate::utils::empty_args;
use crate::Error;

#[derive(Debug, Default)]
pub struct Function {
    pub attr:		Vec<TokenTree>,
    pub decl:		TokenStream,
    pub body:		TokenStream,
    pub name:		String,
    pub is_async:	bool,
    pub no_return:	bool,
    pub ret:		Option<TokenStream>,
}

impl Function {
    pub fn default_return(&self) -> TokenStream {
        match &self.ret {
            None if self.no_return	=> empty_args().into(),
            None	=> TokenStream::new(),
            Some(_r)	=> {
                let mut res = Vec::<TokenTree>::new();

                //println!("ret={_r}");
                res.extend([
                    TokenTree::Ident(Ident::new(CRATE_NAME, Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("DefaultReturn", Span::mixed_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("default_return", Span::mixed_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new()))
                ]);

                res.into_iter().collect()
            }
        }
    }

    pub fn parse(t: TokenStream) -> Result<Self, Error> {
        // println!("{t}");

        let mut res = Function::default();
        let mut iter = t.into_iter();
        let mut decl = Vec::new();
        let mut result = Vec::new();
        let mut has_result = false;
        let mut expect_gt = false;

        // 'pub async fn'
        loop {
            match iter.next() {
                None	=> return Err(Error::FunctionDeclIncomplete),

                // '#[...]'
                Some(TokenTree::Punct(p)) if p.as_char() == '#'	=> {
                    match iter.next() {
                        Some(tt)	=> res.attr.push(tt),
                        None		=> return Err(Error::FunctionDeclBad),
                    }
                },

                Some(ref t @ TokenTree::Ident(ref id))	=> {
                    decl.push(t.clone());

                    match id.to_string().as_str() {
                        "async"	=> res.is_async = true,
                        "fn"	=> break,
                        _	=> {},
                    }
                },

                // e.g. the group in `pub(self)`
                Some(t)	=> decl.push(t.clone()),
            }
        }

        // function name
        match iter.next() {
            None	=> return Err(Error::FunctionDeclIncomplete),
            Some(ref t @ TokenTree::Ident(ref id))	=> {
                decl.push(t.clone());

                res.name = id.to_string();
            },
            Some(_)	=> return Err(Error::FunctionDeclBad),
        }

        // args
        let args = loop {
            match iter.next() {
                None	=> return Err(Error::FunctionDeclIncomplete),
                Some(ref g @ TokenTree::Group(ref grp))
                    if grp.delimiter() == Delimiter::Parenthesis => break g.clone(),
                Some(t)	=> decl.push(t),
            }
        };

        decl.push(args);

        let body = loop {
            match iter.next() {
                None	=> return Err(Error::FunctionDeclIncomplete),

                // handle '->'
                Some(ref punct @ TokenTree::Punct(ref p))
                    if p.as_char() == '-' && !has_result && !expect_gt => {
                        decl.push(punct.clone());
                        expect_gt = true;
                    },

                Some(ref punct @ TokenTree::Punct(ref p))
                    if p.as_char() == '>' && !has_result && expect_gt => {
                        decl.push(punct.clone());
                        expect_gt = false;
                        has_result = true;
                    },

                // handle '!' after '->'
                Some(ref _punct @ TokenTree::Punct(ref p))
                    if p.as_char() == '!' && has_result	=> {
                        // translate '!' to '()';  else our 'skip' will not work
                        //decl.push(punct.clone());
                        decl.push(empty_args());
                        res.no_return = true;
                        break iter.next();
                    },

                Some(grp @ TokenTree::Group(_)) if !has_result	=>
                    break Some(grp.clone()),

                Some(ref grp @ TokenTree::Group(ref g))
                    if !result.is_empty() && g.delimiter() == Delimiter::Brace	=> {
                        res.ret = Some(result.into_iter().collect());
                        break Some(grp.clone());
                    },

                Some(t) if has_result		=> {
                    decl.push(t.clone());
                    result.push(t);
                },

                // TODO: handle 'where' clause

                Some(_)	=> return Err(Error::FunctionDeclBad),
            }
        };

        res.decl = decl.into_iter().collect();
        res.body = match body {
            None	=> return Err(Error::FunctionNoBody),
            Some(b)	=> b.into(),
        };

        if let Some(t) = iter.next() {
            eprintln!("unparsed extra tokens after function: {t}");
            return Err(Error::FunctionExtraTokens);
        }

        Ok(res)
    }
}
