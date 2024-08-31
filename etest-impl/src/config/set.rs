use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

use crate::Error;
use super::ListIterator;

#[derive(Default)]
pub struct TokenSet(Vec<TokenStream>);

impl TokenSet {
    fn from_group(g: Group) -> Result<Self, Error> {
        let mut res = Vec::new();

        for v in ListIterator::new(g.stream()) {
            let v = v.map_err(|_| Error::BadValue)?;

            res.push(v);
        }

        Ok(Self(res))
    }

    pub(super) fn from_stream(tokens: TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();

        let res = match iter.next() {
            None					=> return Err(Error::NoValue),
            Some(l @ TokenTree::Literal(_))		=> Self(vec![l.into()]),

            Some(TokenTree::Group(g))
                if matches!(g.delimiter(), Delimiter::Brace | Delimiter::Bracket)
                => Self::from_group(g)?,

            _		=> return Err(Error::BadValue),
        };

        if let Some(_) = iter.next() {
            return Err(Error::ExtraData);
        }

        Ok(res)
    }

    pub fn push(&mut self, tokens: TokenStream) {
        self.0.push(tokens);
    }
}

impl std::ops::Deref for TokenSet {
    type Target = Vec<TokenStream>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for TokenSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut l = f.debug_list();

        for t in &self.0 {
            l.entry(&format!("{}", t));
        }

        l.finish()
    }
}
