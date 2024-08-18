use proc_macro::{TokenStream, TokenTree};

use crate::utils::err;

use super::ConfigItem;

pub struct ConfigIterator(<TokenStream as IntoIterator>::IntoIter);

impl ConfigIterator {
    pub fn new(t: TokenStream) -> Self {
        t.into()
    }
}

impl From<TokenStream> for ConfigIterator {
    fn from(val: TokenStream) -> Self {
        ConfigIterator(val.into_iter())
    }
}

impl Iterator for ConfigIterator {
    type Item = Result<ConfigItem, TokenStream>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = match self.0.next() {
            None			=> return None,
            Some(TokenTree::Ident(i))	=> i,
            Some(t)			=> return Some(Err(err(t.span(), "unexpected key"))),
        };

        let val = match self.0.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ','	=> None,
            Some(TokenTree::Punct(p)) if p.as_char() == '='	=>
                Some(self.0
                     .by_ref()
                     .take_while(|t| !matches!(t, TokenTree::Punct(p) if p.as_char() == ','))
                     .collect()),
            Some(t)			=> return Some(Err(err(t.span(), "unexpected delimeter"))),
            None			=> None,
        };

        Some(Ok(ConfigItem {
            key:	key,
            val:	val,
        }))
    }
}

pub struct ListIterator(<TokenStream as IntoIterator>::IntoIter);

impl ListIterator {
    pub fn new(t: TokenStream) -> Self {
        t.into()
    }
}

impl From<TokenStream> for ListIterator {
    fn from(val: TokenStream) -> Self {
        ListIterator(val.into_iter())
    }
}

impl Iterator for ListIterator {
    type Item = Result<TokenStream, TokenStream>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = Vec::new();
        let mut is_first = true;
        let mut need_elem = true;

        loop {
            match self.0.next() {
                None if is_first	=> return None,
                None			=> break,

                Some(TokenTree::Punct(p))
                    if p.as_char() == ',' && !need_elem		=> break,

                Some(TokenTree::Punct(p)) if p.as_char() == ','	=>
                    return Some(Err(err(p.span(), "unexpected delimeter"))),

                Some(tt)	=> {
                    res.push(tt);
                    need_elem = false;
                }
            }

            is_first = false;
        }

        Some(Ok(res.into_iter().collect()))
    }
}
