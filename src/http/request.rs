use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: String,
}

impl Request {}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    /*
       GET /user?id=10 HTTP/1.1\r\n
       HEADERS \r\n
       BODY
    */
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
