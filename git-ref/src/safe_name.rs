use crate::ValidPartialName;
use bstr::{BStr, BString, ByteSlice};
use quick_error::quick_error;
use std::{borrow::Cow, convert::TryFrom, path::Path};

quick_error! {
    /// The error used in the [`SafePartialName`]::try_from(…) implementations.
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        RefnameValidation{err: git_validate::reference::name::Error, path: BString} {
            display("The path to a symbolic reference is invalid")
            source(err)
        }
    }
}

impl<'a> ValidPartialName<'a> {
    /// Convert this name into the relative path identifying the reference location.
    pub fn to_path(&self) -> Cow<'a, Path> {
        self.0.to_path_lossy()
    }
}

impl<'a> TryFrom<&'a BStr> for ValidPartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(ValidPartialName(
            git_validate::reference::name_partial(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}

impl<'a> TryFrom<&'a str> for ValidPartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(ValidPartialName(
            git_validate::reference::name_partial(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}

impl<'a> TryFrom<&'a String> for ValidPartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(ValidPartialName(
            git_validate::reference::name_partial(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}
