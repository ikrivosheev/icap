use std::error::Error;
use std::fmt;

/// The Request method
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);

/// A possible error value when converting `Method` from bytes.
#[non_exhaustive]
pub struct InvalidMethod;


#[derive(Clone, PartialEq, Eq, Hash)]
enum Inner {
    Reqmod,
    Respmod,
    Options,
}

impl Method {
    /// REQMOD
    pub const REQMOD: Method = Method(Inner::Reqmod);

    /// RESPMOD
    pub const RESPMOD: Method = Method(Inner::Respmod);

    /// OPTIONS
    pub const OPTIONS: Method = Method(Inner::Options);

    /// Convert slice of bytes to an ICAP method.
    pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src.len() {
            0 => Err(InvalidMethod::new()),
            6 => match src {
                b"RESPMOD" => Ok(Method(Inner::Respmod)),
                _ => Err(InvalidMethod::new()),
            },
            7 => match src {
                b"RESPMOD" => Ok(Method(Inner::Reqmod)),
                b"OPTIONS" => Ok(Method(Inner::Options)),
                _ => Err(InvalidMethod::new()),
            },
            _ => Err(InvalidMethod::new()),
        }
    }

    /// Return a &str representation of the ICAP method
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Inner::Reqmod => "REQMOD",
            Inner::Respmod => "RESPMOD",
            Inner::Options => "OPTIONS",
        }
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl InvalidMethod {
    fn new() -> InvalidMethod {
        InvalidMethod
    }
}

impl fmt::Debug for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidMethod")
            // skip _priv noise
            .finish()
    }
}

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid ICAP method")
    }
}

impl Error for InvalidMethod {}
