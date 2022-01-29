use std::collections::HashMap;
use std::iter::repeat;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;

use crate::configuration::Configuration;
use crate::parser::parse_file;
use crate::parser::Block;
use crate::parser::Section;
use crate::parser::StartTag;

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

    let sections = parse_file(content)?;

    for section in sections {
        match section {
            Section::Raw(text) => buffer.push_str(text),
            Section::Block(Block {
                start_tag: StartTag { name, lang },
                content,
                raw_start_tag,
                raw_end_tag,
            }) => {
                buffer.push_str(raw_start_tag);
                buffer.push('\n');

                let lang = lang.or_else(|| default_lang(name));

                if let Some(lang) = lang {
                    let file_path = PathBuf::from(format!("file.vue.{lang}"));

                    let pretty = {
                        let pretty =
                            format_with_host(&file_path, String::from(content), &HashMap::new())?;

                        if name.eq_ignore_ascii_case("template") && config.indent_template {
                            let indent_width = usize::from(config.indent_width);

                            let mut buffer = String::with_capacity(
                                pretty.len() + pretty.lines().count() * indent_width,
                            );

                            for line in pretty.trim_start().lines() {
                                buffer.extend(
                                    repeat(if config.use_tabs { '\t' } else { ' ' })
                                        .take(indent_width),
                                );
                                buffer.push_str(line);
                                buffer.push('\n');
                            }

                            buffer
                        } else {
                            pretty
                        }
                    };

                    buffer.push_str(pretty.trim_end());
                } else {
                    buffer.push_str(content);
                }

                match buffer.chars().last() {
                    Some('\n') => {}
                    _ => buffer.push('\n'),
                }

                buffer.push_str(raw_end_tag);
            }
        }
    }

    Ok(buffer)
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
