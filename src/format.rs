use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::iter::repeat;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;

use crate::configuration::Configuration;
use vue_sfc::{Block, Section};

fn default_lang(block: &str) -> Option<&'static str> {
    match block {
        "template" => Some("html"),
        "script" => Some("js"),
        "style" => Some("css"),
        _ => None,
    }
}

pub fn format(
    _path: &Path,
    content: &str,
    config: &Configuration,
    mut format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
) -> Result<String> {
    let mut buffer = String::new();

    let mut sections = vue_sfc::parse(content)?.into_iter().peekable();

    while let Some(section) = sections.next() {
        if let Section::Block(block) = section {
            write!(
                &mut buffer,
                "{}",
                format_block(block, config, &mut format_with_host)?
            )?;
        } else {
            write!(&mut buffer, "{}", section)?;
        }

        if sections.peek().is_some() {
            writeln!(&mut buffer)?;
            writeln!(&mut buffer)?;
        }
    }

    Ok(buffer)
}

fn format_block<'a>(
    block: Block<'a>,
    config: &Configuration,
    format_with_host: &mut impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
) -> Result<Block<'a>> {
    let lang = block
        .attributes
        .iter()
        .find_map(|(name, value)| match (name.as_str(), value) {
            ("lang", Some(value)) => Some(value.as_str()),
            _ => None,
        })
        .or_else(|| default_lang(&block.name));

    if let Some(lang) = lang {
        let file_path = PathBuf::from(format!("file.vue.{lang}"));

        let pretty = if block.name.as_str() == "template" && config.indent_template {
            // We compute a hash to check if file content was formatted.
            // If the content was formatted, it is indented.
            // TODO: Remove hash check, blocked by:
            // <https://github.com/dprint/dprint/issues/462>.
            let original_hash = blake3::hash(block.content.as_bytes());

            let pretty = format_with_host(&file_path, block.content.into_owned(), &HashMap::new())?;

            let pretty_hash = blake3::hash(pretty.as_bytes());

            if original_hash == pretty_hash {
                pretty
            } else {
                let indent_width = usize::from(config.indent_width);

                let mut buffer =
                    String::with_capacity(pretty.len() + pretty.lines().count() * indent_width);

                for line in pretty.lines() {
                    buffer.extend(
                        repeat(if config.use_tabs { '\t' } else { ' ' }).take(indent_width),
                    );
                    buffer.push_str(line);
                    buffer.push('\n');
                }

                buffer
            }
        } else {
            format_with_host(&file_path, block.content.into_owned(), &HashMap::new())?
        };

        Ok(Block {
            name: block.name,
            attributes: block.attributes,
            content: Cow::Owned(pretty),
        })
    } else {
        Ok(block)
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use crate::configuration::Configuration;

    use super::format;

    #[test]
    fn test_format_with_host() {
        let config = Configuration {
            indent_template: true,
            use_tabs: false,
            indent_width: 2,
        };

        let raw = "<template></template><script></script>";

        let mut buffer = Vec::new();

        format(Path::new("file.vue"), raw, &config, |path, content, _| {
            buffer.push((path.to_owned(), content.clone()));
            Ok(content)
        })
        .unwrap();

        assert_eq!(buffer[0], (PathBuf::from("file.vue.html"), String::new()));

        assert_eq!(buffer[1], (PathBuf::from("file.vue.js"), String::new()));
    }

    #[test]
    fn test_indent_template() {
        let config = Configuration {
            indent_template: true,
            use_tabs: false,
            indent_width: 2,
        };

        let raw = "<template><div></div></template>";
        let pretty = format(Path::new("file.vue"), raw, &config, |_, raw, _| Ok(raw)).unwrap();

        assert_eq!(pretty, "<template>\n  <div></div>\n</template>");

        let pretty = format(Path::new("file.vue"), &pretty, &config, |_, raw, _| Ok(raw)).unwrap();

        assert_eq!(pretty, "<template>\n  <div></div>\n</template>");
    }
}
