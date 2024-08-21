use proc_macro::{Ident, Span, TokenStream};

use crate::Error;
use crate::utils::err;

use super::TokenSet;

#[derive(Debug)]
pub struct ConfigItem {
    pub(super) key:	Ident,
    pub(super) val:	Option<TokenStream>,
}

impl ConfigItem {
    pub fn get_key(&self) -> String {
        self.key.to_string()
    }

    pub fn convert<T>(&self) -> Result<Option<T>, TokenStream>
    where
        for <'a> &'a Self: TryInto<T>,
        for <'a> <&'a Self as TryInto<T>>::Error: std::fmt::Debug,
    {
        Ok(Some(self.try_into()
                .map_err(|e| err(Span::call_site(),
                                 &format!("bad '{}' value: {e:?}", self.get_key())))?))
    }
}

impl TryInto<TokenSet> for &ConfigItem {
    type Error = Error;

    fn try_into(self) -> Result<TokenSet, Self::Error> {
        TokenSet::from_stream(self.try_into()?)
    }
}

impl TryInto<TokenStream> for &ConfigItem {
    type Error = Error;

    fn try_into(self) -> Result<TokenStream, Self::Error> {
        self.val.as_ref().ok_or(Error::NoValue).cloned()
    }
}
