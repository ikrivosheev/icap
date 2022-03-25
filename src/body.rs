pub struct Body<T>(Inner<T>);

enum Inner<T> {
    Request(T),
    Response(T),
    Options(T),
}

impl<T> Body<T> {
    pub fn request(body: T) -> Body<T> {
        Body(Inner::Request(body))
    }

    pub fn response(body: T) -> Body<T> {
        Body(Inner::Response(body))
    }

    pub fn options(body: T) -> Body<T> {
        Body(Inner::Options(body))
    }

    pub fn into_inner(self) -> T {
        match self.0 {
            Inner::Request(body) | Inner::Response(body) | Inner::Options(body) => body,
        }
    }
}

impl<T> AsRef<T> for Body<T> {
    fn as_ref(&self) -> &T {
        match self.0 {
            Inner::Request(ref body) | Inner::Response(ref body) | Inner::Options(ref body) => body,
        }
    }
}

impl<T> AsMut<T> for Body<T> {
    fn as_mut(&mut self) -> &mut T {
        match self.0 {
            Inner::Request(ref mut body)
            | Inner::Response(ref mut body)
            | Inner::Options(ref mut body) => body,
        }
    }
}
