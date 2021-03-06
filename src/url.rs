use std::unimplemented;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, char},
    character::is_digit,
    combinator::opt,
    multi::{many0, many1},
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
    pub path: Option<Path>,
    pub query: Option<Vec<Query>>,
    pub fragment_id: Option<FragmentId>,
}

fn parse_scheme(input: &str) -> IResult<&str, Scheme> {
    let (input, scheme) = nom::branch::alt((tag("http://"), tag("https://")))(input)?;

    match scheme {
        "http://" => Ok((input, Scheme::HTTP)),
        "https://" => Ok((input, Scheme::HTTPS)),
        _ => unimplemented!("no other schemes supported"),
    }
}

fn parse_host(input: &str) -> IResult<&str, Host> {
    let (input, host) = tuple((many1(terminated(alphanumeric1, char('.'))), alpha1))(input)?;
    Ok((input, Host(format!("{}.{}", host.0.join("."), host.1))))
}

fn parse_port(input: &str) -> IResult<&str, Port> {
    let (input, _) = tag(":")(input)?;
    let (input, port): (&str, &str) = take_while(|c| is_digit(c as u8))(input)?;
    Ok((input, Port(port.parse::<u32>().unwrap())))
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    let (input, path) = many1(tuple((char('/'), alphanumeric1)))(input)?;

    Ok((
        input,
        Path(
            path.iter()
                .map(|(_, v)| format!("/{}", v))
                .collect::<Vec<_>>()
                .join(""),
        ),
    ))
}

fn parse_query(input: &str) -> IResult<&str, Vec<Query>> {
    let query_with = |s| tuple((tag(s), alphanumeric1, char('='), alphanumeric1));
    let (input, query) = tuple((query_with("?"), many0(query_with("&"))))(input)?;

    let mut queries = Vec::new();
    queries.push(Query(query.0 .1.to_string(), query.0 .3.to_string()));
    for q in query.1 {
        queries.push(Query(q.1.to_string(), q.3.to_string()));
    }

    Ok((input, queries))
}

fn parse_fragment_id(input: &str) -> IResult<&str, FragmentId> {
    let (input, fragment_id) = tuple((tag("#"), alphanumeric1))(input)?;
    Ok((input, FragmentId(fragment_id.1.to_string())))
}

pub fn parse_url(input: &str) -> IResult<&str, URL> {
    let (input, url) = tuple((
        parse_scheme,
        parse_host,
        opt(parse_port),
        opt(parse_path),
        opt(parse_query),
        opt(parse_fragment_id),
    ))(input)?;

    Ok((
        input,
        URL {
            scheme: url.0,
            host: url.1,
            port: url.2,
            path: url.3,
            query: url.4,
            fragment_id: url.5,
        },
    ))
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

#[test]
fn test_parse_port() {
    assert_eq!(parse_port(":0/a"), Ok(("/a", Port(0))));
    assert_eq!(parse_port(":65535/a"), Ok(("/a", Port(65535))));
    assert_eq!(opt(parse_port)("/a"), Ok(("/a", None)));
}

#[test]
fn test_parse_path() {
    assert_eq!(
        parse_path("/a/b?id=0"),
        Ok(("?id=0", Path("/a/b".to_string())))
    );
    assert_eq!(parse_path("/a?id=0"), Ok(("?id=0", Path("/a".to_string()))));
    assert_eq!(opt(parse_path)("?id=0"), Ok(("?id=0", None)));
}

#[test]
fn test_parse_query() {
    assert_eq!(
        parse_query("?a=0#a"),
        Ok(("#a", vec![Query("a".to_string(), "0".to_string())]))
    );
    assert_eq!(
        parse_query("?a=0&b=1#a"),
        Ok((
            "#a",
            vec![
                Query("a".to_string(), "0".to_string()),
                Query("b".to_string(), "1".to_string())
            ]
        ))
    );
    assert_eq!(opt(parse_query)("#a"), Ok(("#a", None)));
}

#[test]
fn test_parse_fragment_id() {
    assert_eq!(
        parse_fragment_id("#a"),
        Ok(("", FragmentId("a".to_string())))
    );
    assert_eq!(opt(parse_fragment_id)(""), Ok(("", None)));
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
                path: Some(Path("/a/b".to_string())),
                query: Some(vec!(Query("id".to_string(), "10".to_string()))),
                fragment_id: Some(FragmentId("Index".to_string()))
            }
        ))
    );
}
