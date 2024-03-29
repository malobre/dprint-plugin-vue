use std::path::Path;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;

use crate::configuration::Configuration;

pub struct VuePluginHandler;

impl VuePluginHandler {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        VuePluginHandler
    }
}

impl SyncPluginHandler<Configuration> for VuePluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            config_key: String::from("vue"),
            file_extensions: vec![String::from("vue")],
            file_names: vec![],
            help_url: String::from("https://github.com/malobre/dprint-plugin-vue"),
            config_schema_url: String::new(),
            update_url: Some(String::from(
                "https://plugins.dprint.dev/malobre/vue/latest.json",
            )),
        }
    }

    fn license_text(&mut self) -> String {
        String::from(include_str!("../LICENSE"))
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        Configuration::resolve(config, global_config)
    }

    fn format(
        &mut self,
        file_path: &Path,
        file_text: &str,
        config: &Configuration,
        format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<Option<String>>,
    ) -> Result<Option<String>> {
        crate::format::format(file_path, file_text, config, format_with_host).map(Some)
    }
}
