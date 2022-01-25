use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::plugins::PluginHandler;
use dprint_core::plugins::PluginInfo;

use crate::configuration::{self, Configuration};
use crate::parser::parse_file;
use crate::parser::Block;
use crate::parser::Section;

pub struct VuePluginHandler;

impl VuePluginHandler {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        VuePluginHandler
    }

    fn default_lang(block: &str) -> Option<&'static str> {
        match block {
            "template" => Some("html"),
            "script" => Some("js"),
            "style" => Some("css"),
            _ => None,
        }
    }
}

impl PluginHandler<Configuration> for VuePluginHandler {
    fn get_plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: String::from("vue"),
            file_extensions: vec![String::from("vue")],
            file_names: vec![],
            help_url: String::new(),
            config_schema_url: String::new(),
        }
    }

    fn get_license_text(&mut self) -> String {
        String::from(include_str!("../LICENSE"))
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        configuration::resolve_config(config, global_config)
    }

    fn format_text(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        _config: &Configuration,
        mut format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
    ) -> Result<String> {
        let mut buffer = String::new();

        let sections = parse_file(file_text)?;

        for section in sections {
            match section {
                Section::Root(value) => buffer.push_str(value),
                Section::Block(Block {
                    start_tag,
                    content,
                    raw_start_tag,
                    raw_end_tag,
                }) => {
                    buffer.push_str(raw_start_tag);
                    buffer.push('\n');

                    let lang = start_tag
                        .lang
                        .or_else(|| Self::default_lang(start_tag.name));

                    if let Some(lang) = lang {
                        let file_path = PathBuf::from(format!("file.{}", lang));

                        buffer.push_str(&format_with_host(
                            &file_path,
                            String::from(content),
                            &HashMap::new(),
                        )?);
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
}
