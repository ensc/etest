use proc_macro::{Delimiter, Literal, Span, TokenStream, TokenTree};

use crate::defs::*;
use crate::utils::err;

use super::{ TokenSet, ConfigIterator };

#[derive(Default, Debug)]
pub struct Config {
    pub test_fn:	Option<TokenStream>,
    pub skip_fn:	Option<TokenStream>,
    pub skip_result:	Option<TokenStream>,
    pub timeout:	Option<TokenStream>,
    pub uses:		TokenSet,
    pub consumes:	TokenSet,
}

impl Config {
    fn get_default_uses() -> TokenSet {
        TokenSet::from_stream(vec![
            TokenTree::Literal(Literal::string(CPU_RESOURCE))
        ].into_iter().collect()).unwrap()
    }

    pub fn parse(attr: TokenStream) -> Result<Config, TokenStream> {
        let mut res = Config::default();
        let mut no_default_uses = false;

        for cfg in ConfigIterator::new(attr) {
            let cfg = cfg?;

            match cfg.get_key().as_str() {
                "no_default_uses"	=> no_default_uses   = true,
                "test_fn"	=> res.test_fn       = cfg.convert::<TokenStream>()?,
                "skip"		=> res.skip_fn       = cfg.convert::<TokenStream>()?,
                "skip_result"	=> res.skip_result   = cfg.convert::<TokenStream>()?,
                "timeout"	=> res.timeout       = cfg.convert::<TokenStream>()?,
                "uses"		=> res.uses          = cfg.convert::<TokenSet>()?.unwrap(),
                "consumes"	=> res.consumes      = cfg.convert::<TokenSet>()?.unwrap(),
                c		=> return Err(err(Span::call_site(), &format!("unsupported key: {c:?}")))
            }
        }

        if !no_default_uses {
            res.uses.extend(Config::get_default_uses());
        }

        Ok(res)
    }

    // checks whether the test_fn is not '()'
    pub(super) fn has_test_fn(&self) -> bool {
        let Some(func) = &self.test_fn else {
            // when not specified, try to guess the test function
            return true;
        };

        let mut iter = func.clone().into_iter();

        match iter.next() {
            Some(TokenTree::Group(g))
                if g.delimiter() == Delimiter::Parenthesis && g.stream().is_empty()	=> {},

            _	=> return true,
        }

        iter.next().is_some()
    }
}
