use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while, take_while1},
    character::complete::{char, newline},
    combinator::{consumed, flat_map, opt, recognize},
    error::ErrorKind,
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

use super::util::is_ascii_whitespace;

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub start_tag: StartTag<'a>,
    pub raw_start_tag: &'a str,
    pub raw_end_tag: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StartTag<'a> {
    pub name: &'a str,
    pub lang: Option<&'a str>,
}

/// See <https://html.spec.whatwg.org/multipage/syntax.html#attributes-2>.
fn parse_attribute_name(input: &str) -> IResult<&str, &str> {
    take_while1(|char: char| {
        !matches!(char,
        '\u{007F}'..='\u{009F}'
        | '\u{0020}'
        | '\u{0022}'
        | '\u{0027}'
        | '\u{003E}'
        | '\u{002F}'
        | '\u{003D}'
        | '\u{FDD0}'..='\u{FDEF}'
        | '\u{FFFE}'
        | '\u{FFFF}'
        | '\u{1FFFE}'
        | '\u{1FFFF}'
        | '\u{2FFFE}'
        | '\u{2FFFF}'
        | '\u{3FFFE}'
        | '\u{3FFFF}'
        | '\u{4FFFE}'
        | '\u{4FFFF}'
        | '\u{5FFFE}'
        | '\u{5FFFF}'
        | '\u{6FFFE}'
        | '\u{6FFFF}'
        | '\u{7FFFE}'
        | '\u{7FFFF}'
        | '\u{8FFFE}'
        | '\u{8FFFF}'
        | '\u{9FFFE}'
        | '\u{9FFFF}'
        | '\u{AFFFE}'
        | '\u{AFFFF}'
        | '\u{BFFFE}'
        | '\u{BFFFF}'
        | '\u{CFFFE}'
        | '\u{CFFFF}'
        | '\u{DFFFE}'
        | '\u{DFFFF}'
        | '\u{EFFFE}'
        | '\u{EFFFF}'
        | '\u{FFFFE}'
        | '\u{FFFFF}'
        | '\u{10FFFE}'
        | '\u{10FFFF}')
    })(input)
}

fn parse_attribute(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    pair(
        parse_attribute_name,
        opt(preceded(
            tuple((
                take_while(is_ascii_whitespace),
                char('='),
                take_while(is_ascii_whitespace),
            )),
            alt((
                delimited(char('"'), take_until("\""), char('"')),
                delimited(char('\''), take_until("'"), char('\'')),
            )),
        )),
    )(input)
}

/// See <https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name>.
fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    take_till(|char: char| char.is_ascii_whitespace() || char == '/' || char == '>')(input)
}

/// See <https://html.spec.whatwg.org/multipage/syntax.html#start-tags>.
fn parse_start_tag(input: &str) -> IResult<&str, StartTag> {
    let (input, _) = char('<')(input)?;

    let (input, name) = parse_tag_name(input)?;

    let (input, attributes) =
        many0(preceded(take_while(is_ascii_whitespace), parse_attribute))(input)?;

    let lang = attributes
        .into_iter()
        .find_map(|attribute| match attribute {
            ("lang", Some(lang)) => Some(lang),
            _ => None,
        });

    let (input, _) = tuple((take_while(is_ascii_whitespace), opt(char('/')), char('>')))(input)?;

    Ok((input, StartTag { name, lang }))
}

fn parse_tag_content<'a>(tag_name: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &str| {
        let mut nesting_level = 0u16;
        let mut index = match input.find('<') {
            Some(index) => index,
            None => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    ErrorKind::TakeUntil,
                )))
            }
        };

        while !input[index..].is_empty() {
            if let Ok((_, start_tag)) = parse_start_tag(&input[index..]) {
                if start_tag.name == tag_name {
                    nesting_level += 1;
                }
            } else if let Ok((_, _)) = parse_end_tag(tag_name)(&input[index..]) {
                if nesting_level == 0 {
                    return Ok((&input[index..], &input[..index]));
                }

                nesting_level -= 1;
            }

            index += match input.get((index + 1)..).and_then(|input| input.find('<')) {
                Some(index) => index + 1,
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        ErrorKind::TakeUntil,
                    )))
                }
            };
        }

        Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::TakeUntil,
        )))
    }
}

/// Parse an end tag with the given `tag_name`.
/// See <https://html.spec.whatwg.org/multipage/syntax.html#end-tags>.
fn parse_end_tag<'a>(tag_name: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(
        tag("</"),
        tag(tag_name),
        tuple((take_while(is_ascii_whitespace), char('>'))),
    )
}

pub fn parse_block(input: &str) -> IResult<&str, Block> {
    flat_map(
        terminated(consumed(parse_start_tag), opt(newline)),
        |(raw_start_tag, start_tag)| {
            let tag_name = start_tag.name;

            tuple((
                parse_tag_content(tag_name),
                recognize(parse_end_tag(tag_name)),
            ))
            .map(move |(content, raw_end_tag)| Block {
                start_tag,
                raw_start_tag,
                raw_end_tag,
                content,
            })
        },
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use super::{
        parse_attribute, parse_attribute_name, parse_block, parse_end_tag, parse_start_tag,
        parse_tag_content, parse_tag_name, Block, StartTag,
    };

    #[test]
    fn test_parse_attribute_name() {
        assert_eq!(
            parse_attribute_name(r#"lang="ts" setup>"#),
            Ok((r#"="ts" setup>"#, "lang"))
        );

        assert_eq!(parse_attribute_name("setup>"), Ok((">", "setup")));

        assert!(parse_attribute_name("> text").is_err(),);
    }

    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            parse_attribute(r#"lang="ts" setup>"#),
            Ok((" setup>", ("lang", Some("ts"))))
        );

        assert_eq!(parse_attribute("setup>"), Ok((">", ("setup", None))));
    }

    #[test]
    fn test_parse_tag_name() {
        assert_eq!(parse_tag_name("script>"), Ok((">", "script")));
        assert_eq!(parse_tag_name("script >"), Ok((" >", "script")));
        assert_eq!(parse_tag_name("script\t>"), Ok(("\t>", "script")));
        assert_eq!(parse_tag_name("script \t>"), Ok((" \t>", "script")));
        assert_eq!(
            parse_tag_name(r#"script lang="ts">"#),
            Ok((r#" lang="ts">"#, "script"))
        );
    }

    #[test]
    fn test_parse_start_tag() {
        assert_eq!(
            parse_start_tag(r#"<script lang="ts" setup>"#),
            Ok((
                "",
                StartTag {
                    name: "script",
                    lang: Some("ts")
                }
            ))
        );
    }

    #[test]
    fn test_parse_end_tag() {
        assert_eq!(parse_end_tag("script")("</script>"), Ok(("", "script")));
        assert_eq!(parse_end_tag("script")("</script >"), Ok(("", "script")));
        assert_eq!(parse_end_tag("script")("</script\t>"), Ok(("", "script")));
        assert_eq!(parse_end_tag("script")("</script \t>"), Ok(("", "script")));
    }

    #[test]
    fn test_parse_tag_content() {
        assert_eq!(
            parse_tag_content("script")("let value = true;\nconsole.log(value);\n</script>"),
            Ok(("</script>", "let value = true;\nconsole.log(value);\n"))
        );

        assert_eq!(
            parse_tag_content("script")(
                "let value = Math.random();\nconsole.log(value < 0.5);\n</script>"
            ),
            Ok((
                "</script>",
                "let value = Math.random();\nconsole.log(value < 0.5);\n"
            ))
        );

        assert_eq!(
            parse_tag_content("template")("<template></template></template>"),
            Ok(("</template>", "<template></template>"))
        );
    }

    #[test]
    fn test_parse_block() {
        assert_eq!(
            parse_block("<script>\nlet value = true;\nconsole.log(value);\n</script>\n<!-- residual data -->"),
            Ok((
                "\n<!-- residual data -->",
                Block {
                    start_tag: StartTag {
                        name: "script",
                        lang: None
                    },
                    raw_start_tag: "<script>",
                    raw_end_tag: "</script>",
                    content: "let value = true;\nconsole.log(value);\n"
                }
            ))
        );
    }
}
