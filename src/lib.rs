use {headers::HeaderMapExt, reqwest::RequestBuilder};

pub trait RequestBuilderExt: private::Sealed + Sized {
    fn typed_header<H>(self, header: H) -> Self
    where
        H: headers::Header;
}

impl RequestBuilderExt for RequestBuilder {
    fn typed_header<H>(self, header: H) -> Self
    where
        H: headers::Header,
    {
        let mut headers = http::HeaderMap::new();
        headers.typed_insert(header);
        self.headers(headers)
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for RequestBuilder {}
}
