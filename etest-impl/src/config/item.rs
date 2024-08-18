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

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    pub fn get_val(&self) -> Result<TokenStream, Error> {
        self.val
            .as_ref()
            .ok_or(Error::NoValue)
            .cloned()
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

impl TryInto<u64> for &ConfigItem {
    type Error = Error;

    fn try_into(self) -> Result<u64, Self::Error> {
        let Some(v) = &self.val else {
            return Err(Error::NoValue);
        };

        let mut iter = v.clone().into_iter();

        match iter.next().map(litrs::Literal::try_from) {
            None					=> Err(Error::NoValue),
            Some(Ok(litrs::Literal::Integer(i)))	=> i.value().ok_or(Error::BadInteger),
            Some(Ok(_))					=> Err(Error::BadType),
            Some(Err(e))				=> {
                eprintln!("failed to convert literal: {e:?}");
                Err(Error::BadValue)
            }
        }
    }
}

impl TryInto<String> for &ConfigItem {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let Some(v) = &self.val else {
            return Err(Error::NoValue);
        };

        let mut iter = v.clone().into_iter();

        match iter.next().map(litrs::Literal::try_from) {
            None					=> Err(Error::NoValue),
            Some(Ok(litrs::Literal::String(i)))		=> Ok(i.value().to_owned()),
            Some(Ok(_))					=> Err(Error::BadType),
            Some(Err(e))				=> {
                eprintln!("failed to convert literal: {e:?}");
                Err(Error::BadValue)
            }
        }
    }
}
