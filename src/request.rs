use crate::body::Body;
use crate::method::Method;
use crate::version::Version;

use http::request::Parts as HttpRequestParts;
use http::response::Parts as HttpResponseParts;
use http::{HeaderMap, HeaderValue, Uri};

pub struct Request<T> {
    head: Parts,

    http_request: Option<HttpRequestParts>,
    http_response: Option<HttpResponseParts>,

    body: Option<Body<T>>,
}

#[non_exhaustive]
pub struct Parts {
    /// The request's method
    pub method: Method,

    /// The request's uri
    pub uri: Uri,

    /// The request's version
    pub version: Version,

    /// The request's headers
    pub headers: HeaderMap<HeaderValue>,
}

impl Request<()> {
    pub fn reqmode<T>(uri: T, http_request: Option<HttpRequestParts>) {}

    pub fn respmode<T>(uri: T, http_response: Option<HttpResponseParts>) {}

    pub fn options<T>(uri: T) {}
}

impl<T> Request<T> {
    pub fn method(&self) -> &Method {
        &self.head.method
    }

    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.head.method
    }

    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }

    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.head.uri
    }

    pub fn version(&self) -> &Version {
        &self.head.version
    }

    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.head.version
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.head.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.head.headers
    }

    pub fn body(&self) -> &Option<Body<T>> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Option<Body<T>> {
        &mut self.body
    }

    pub fn into_body(self) -> Option<T> {
        self.body.map(|b| b.into_inner())
    }
}
