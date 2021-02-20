use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

#[derive(Debug, PartialEq)]
pub struct Host(String);

#[derive(Debug, PartialEq)]
pub struct Port(u32);

#[derive(Debug, PartialEq)]
pub struct Path(String);

#[derive(Debug, PartialEq)]
pub struct Query(String, String);

#[derive(Debug, PartialEq)]
pub struct FragmentId(String);

#[derive(Debug, PartialEq)]
pub struct URL {
    pub scheme: Scheme,
    pub host: Host,
    pub port: Option<Port>,
    pub path: Path,
    pub query: Vec<Query>,
    pub fragment_id: Option<FragmentId>,
}

pub fn parse_url(_: &str) -> IResult<&str, URL> {
    unimplemented!()
}

#[test]
fn parse_url_test() {
    assert_eq!(
        parse_url("https://example.com:80/a/b?id=10#Index"),
        Ok((
            "",
            URL {
                scheme: Scheme::HTTPS,
                host: Host("example.com".to_string()),
                port: Some(Port(80)),
                path: Path("/a/b".to_string()),
                query: vec!(Query("id".to_string(), "10".to_string())),
                fragment_id: Some(FragmentId("Index".to_string()))
            }
        ))
    );
}
