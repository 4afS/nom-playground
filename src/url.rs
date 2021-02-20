use nom::combinator::map_res;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1},
};
use nom::{
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

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

fn parse_scheme(input: &str) -> IResult<&str, Scheme> {
    let (input, scheme) = nom::branch::alt((
        nom::bytes::complete::tag_no_case("http://"),
        nom::bytes::complete::tag_no_case("https://"),
    ))(input)?;

    match scheme {
        "http://" => Ok((input, Scheme::HTTP)),
        "https://" => Ok((input, Scheme::HTTPS)),
        _ => unimplemented!("no other schemes supported"),
    }
}

#[test]
fn test_parse_scheme() {
    assert_eq!(
        parse_scheme("http://example.com"),
        Ok(("example.com", Scheme::HTTP))
    );
    assert_eq!(
        parse_scheme("https://example.com"),
        Ok(("example.com", Scheme::HTTPS))
    );
}

fn parse_host(input: &str) -> IResult<&str, Host> {
    let (input, host) = tuple((many1(terminated(alphanumeric1, char('.'))), alpha1))(input)?;

    Ok((input, Host(format!("{}.{}", host.0.join("."), host.1))))
    // Ok((input, Host(host.1.to_string())))
}

#[test]
fn test_parse_host() {
    assert_eq!(
        parse_host("host.example.com/a"),
        Ok(("/a", Host("host.example.com".to_string())))
    );
    assert_eq!(
        parse_host("example.com/a"),
        Ok(("/a", Host("example.com".to_string())))
    );
}

fn parse_port(input: &str) -> IResult<&str, Option<Port>> {
    let (input, _) = nom::combinator::opt(tag(":"))(input)?;
    let (input, port): (&str, Option<&str>) = nom::combinator::opt(digit1)(input)?;
    Ok((
        input,
        port.and_then(|s: &str| -> Option<Port> {
            match s.to_string().parse::<u32>() {
                Ok(n) => Some(Port(n)),
                _ => None,
            }
        }),
    ))
}

#[test]
fn test_parse_port() {
    assert_eq!(parse_port(":80/a"), Ok(("/a", Some(Port(80)))));
    assert_eq!(parse_port("/a"), Ok(("/a", None)));
}

pub fn parse_url(_: &str) -> IResult<&str, URL> {
    unimplemented!()
}

#[test]
fn test_parse_url() {
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
