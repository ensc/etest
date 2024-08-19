use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

use crate::Error;
use super::ListIterator;

#[derive(Default, Debug)]
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

        Ok(res)
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl std::ops::Deref for TokenSet {
    type Target = Vec<TokenStream>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TokenSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
