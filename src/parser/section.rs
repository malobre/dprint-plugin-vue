use nom::{
    branch::alt,
    combinator::{peek, recognize, rest},
    multi::many_till,
    IResult, Parser,
};

use super::{block::parse_block, util::take_until_next, Block};

/// Represent the sections of a Vue SFC.
#[derive(Debug, PartialEq)]
pub enum Section<'a> {
    /// Represent any data before, after or between blocks.
    Root(&'a str),
    /// See [`Block`].
    Block(Block<'a>),
}

pub fn parse_section(input: &str) -> IResult<&str, Section> {
    alt((
        parse_block.map(Section::Block),
        alt((
            recognize(many_till(take_until_next("<"), peek(parse_block))),
            rest,
        ))
        .map(Section::Root),
    ))(input)
}

#[cfg(test)]
mod test {
    use crate::parser::block::{Block, StartTag};

    use super::{parse_section, Section};

    #[test]
    fn test_parse_section() {
        assert_eq!(
            parse_section(
                "<!-- A comment -->\n<script>\nlet value = true;\nconsole.log(value);\n</script>"
            ),
            Ok((
                "<script>\nlet value = true;\nconsole.log(value);\n</script>",
                Section::Root("<!-- A comment -->\n")
            ))
        );

        assert_eq!(
            parse_section("<script>\nlet value = true;\nconsole.log(value);\n</script>"),
            Ok((
                "",
                Section::Block(Block {
                    start_tag: StartTag {
                        name: "script",
                        lang: None
                    },
                    raw_start_tag: "<script>",
                    raw_end_tag: "</script>",
                    content: "let value = true;\nconsole.log(value);\n"
                })
            ))
        );

        assert_eq!(
            parse_section("<!-- A comment -->"),
            Ok(("", Section::Root("<!-- A comment -->")))
        );
    }
}
