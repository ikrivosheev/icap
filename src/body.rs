#[derive(Debug)]
pub struct Body<T>(Inner<T>);

#[derive(Debug)]
enum Inner<T> {
    Empty,
    Request(T),
    Response(T),
    Options(T),
}

impl<T> Default for Body<T> {
    fn default() -> Self {
        Body::new()
    }
}

impl<T> Body<T> {
    fn new() -> Body<T> {
        Body(Inner::Empty)
    }

    pub fn request(body: T) -> Body<T> {
        Body(Inner::Request(body))
    }

    pub fn response(body: T) -> Body<T> {
        Body(Inner::Response(body))
    }

    pub fn options(body: T) -> Body<T> {
        Body(Inner::Options(body))
    }

    pub fn into_inner(self) -> Option<T> {
        match self.0 {
            Inner::Empty => None,
            Inner::Request(body) | Inner::Response(body) | Inner::Options(body) => Some(body),
        }
    }

    fn as_ref(&self) -> Option<&T> {
        match self.0 {
            Inner::Empty => None,
            Inner::Request(ref body) | Inner::Response(ref body) | Inner::Options(ref body) => {
                Some(body)
            }
        }
    }

    fn as_mut(&mut self) -> Option<&mut T> {
        match self.0 {
            Inner::Empty => None,
            Inner::Request(ref mut body)
            | Inner::Response(ref mut body)
            | Inner::Options(ref mut body) => Some(body),
        }
    }
}
