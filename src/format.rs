use std::borrow::Cow;
use std::cmp::Ordering;
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
        let pretty = {
            let file_path = PathBuf::from(format!("file.vue.{lang}"));
            let pretty = format_with_host(&file_path, block.content.into_owned(), &HashMap::new())?;

            let indent_width = pretty
                .lines()
                .filter_map(|line| {
                    let trimed_line = line.trim_start();
                    if trimed_line.is_empty() {
                        None
                    } else {
                        Some(line.len() - trimed_line.len())
                    }
                })
                .min()
                .unwrap_or(0);

            let desired_indent_width =
                if block.name.as_str() == "template" && config.indent_template {
                    usize::from(config.indent_width)
                } else {
                    0
                };

            match indent_width.cmp(&desired_indent_width) {
                Ordering::Equal => pretty,
                Ordering::Less => {
                    let delta = desired_indent_width - indent_width;

                    let mut buffer =
                        String::with_capacity(pretty.len() + pretty.lines().count() * delta);

                    for line in pretty.lines() {
                        buffer.extend(repeat(if config.use_tabs { '\t' } else { ' ' }).take(delta));
                        buffer.push_str(line);
                        buffer.push('\n');
                    }

                    buffer
                }
                Ordering::Greater => {
                    let delta = indent_width - desired_indent_width;

                    let mut buffer =
                        String::with_capacity(pretty.len() - pretty.lines().count() * delta);

                    for line in pretty.lines() {
                        buffer.push_str(&line[delta..]);
                        buffer.push('\n');
                    }

                    buffer
                }
            }
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

        assert_eq!(
            format(
                Path::new("file.vue"),
                "<template><div></div></template>",
                &config,
                |_, raw, _| Ok(raw)
            )
            .unwrap(),
            "<template>\n  <div></div>\n</template>"
        );

        assert_eq!(
            format(
                Path::new("file.vue"),
                "<template>\n  <div></div>\n\n  <div></div>\n</template>",
                &config,
                |_, raw, _| Ok(raw)
            )
            .unwrap(),
            "<template>\n  <div></div>\n\n  <div></div>\n</template>"
        );
    }
}
