use std::fmt;
use std::num::NonZeroU16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

pub struct InvalidStatusCode {
    _priv: (),
};

impl StatusCode {
    pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        if src < 100 || src >= 1000 {
            return Err(InvalidStatusCode::new());
        }

        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    /// Converts a &[u8] to a status code
    pub fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        if src.len() != 3 {
            return Err(InvalidStatusCode::new());
        }

        let a = src[0].wrapping_sub(b'0') as u16;
        let b = src[1].wrapping_sub(b'0') as u16;
        let c = src[2].wrapping_sub(b'0') as u16;

        if a == 0 || a > 9 || b > 9 || c > 9 {
            return Err(InvalidStatusCode::new());
        }

        let status = (a * 100) + (b * 10) + c;
        NonZeroU16::new(status)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0.get())
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            u16::from(*self),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

status_codes! {
    /// 100 Continue after ICAP Preview
    (100, CONTINUE, "Continue");

    /// 204 No modifications needed
    (204, NO_CONTENT, "No content");

    /// 400 Bad Request
    (400, BAD_REQUEST, "Bad Request");

    /// 404 ICAP Service not found
    (404, NOT_FOUND, "Not Found");

    /// 405 Method not allowed for service
    (405, METHOD_NOT_ALLOWED, "Method not allower");
    
    /// 408 Request timeout.  ICAP server gave up waiting for a request from an ICAP client.
    (408, REQUEST_TIMEOUT, "Request Timeout");

    /// 500 Internal Server Error
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error");

    /// 501 Not Implemented
    (501, NOT_IMPLEMENTED, "Not Implemented");

    /// 502 Bad Gateway
    (502, BAD_GATEWAY, "Bad Gateway");

    /// 503 Service Unavailable
    (503, SERVICE_UNAVAILABLE, "Service Unavailable");

    /// 505 ICAP Version Not Supported 
    (505, ICAP_VERSION_NOT_SUPPORTED, "ICAP Version Not Supported");

}
