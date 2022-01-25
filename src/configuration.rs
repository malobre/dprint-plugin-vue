use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::NewLineKind;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::configuration::DEFAULT_GLOBAL_CONFIGURATION;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub use_tabs: bool,
    pub indent_width: u8,
    pub new_line_kind: NewLineKind,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            line_width: DEFAULT_GLOBAL_CONFIGURATION.line_width,
            use_tabs: DEFAULT_GLOBAL_CONFIGURATION.use_tabs,
            indent_width: DEFAULT_GLOBAL_CONFIGURATION.indent_width,
            new_line_kind: DEFAULT_GLOBAL_CONFIGURATION.new_line_kind,
        }
    }
}

pub fn resolve_config(
    config: ConfigKeyMap,
    _global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: Configuration::default(),
        diagnostics,
    }
}
