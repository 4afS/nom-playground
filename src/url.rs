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
