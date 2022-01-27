mod block;
mod section;
mod util;

pub use block::{Block, StartTag};
pub use section::Section;

pub fn parse_file(mut input: &str) -> Result<Vec<Section>, anyhow::Error> {
    let mut buffer = Vec::new();

    loop {
        if input.is_empty() {
            break;
        }

        let (remaining, section) =
            section::parse_section(input).map_err(|err| anyhow::Error::from(err.to_owned()))?;

        buffer.push(section);

        input = remaining;
    }

    Ok(buffer)
}

#[cfg(test)]
mod test {
    use crate::parser::{
        block::{Block, StartTag},
        section::Section,
    };

    use super::parse_file;

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file(
                "<!-- A comment -->\n<script>\nlet value = true;\nconsole.log(value);\n</script>"
            )
            .unwrap(),
            vec![
                Section::Raw("<!-- A comment -->\n"),
                Section::Block(Block {
                    start_tag: StartTag {
                        name: "script",
                        lang: None
                    },
                    raw_start_tag: "<script>",
                    raw_end_tag: "</script>",
                    content: "let value = true;\nconsole.log(value);\n"
                })
            ]
        );
    }
}
