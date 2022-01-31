use std::path::Path;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::plugins::PluginHandler;
use dprint_core::plugins::PluginInfo;

use crate::configuration::Configuration;

pub struct VuePluginHandler;

impl VuePluginHandler {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        VuePluginHandler
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
            help_url: "https://github.com/malobre/dprint-plugin-vue/".to_string(),
            config_schema_url: String::new(),
            update_url: Some("https://plugins.dprint.dev/malobre/dprint-plugin-vue/latest.json".to_string()),
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
        Configuration::resolve(config, global_config)
    }

    fn format_text(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
    ) -> Result<String> {
        crate::format::format(file_text, config, format_with_host)
    }
}
