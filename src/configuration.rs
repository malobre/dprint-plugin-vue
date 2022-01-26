use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::configuration::DEFAULT_GLOBAL_CONFIGURATION;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub indent_template: bool,
    pub use_tabs: bool,
    pub indent_width: u8,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            indent_template: true,
            use_tabs: DEFAULT_GLOBAL_CONFIGURATION.use_tabs,
            indent_width: DEFAULT_GLOBAL_CONFIGURATION.indent_width,
        }
    }
}

impl Configuration {
    pub(crate) fn resolve(
        mut config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        let mut diagnostics = Vec::new();

        let resolved_config = Configuration {
            indent_template: get_value(&mut config, "indentTemplate", true, &mut diagnostics),
            use_tabs: get_value(
                &mut config,
                "useTabs",
                global_config
                    .use_tabs
                    .unwrap_or(DEFAULT_GLOBAL_CONFIGURATION.use_tabs),
                &mut diagnostics,
            ),
            indent_width: get_value(
                &mut config,
                "indentWidth",
                global_config
                    .indent_width
                    .unwrap_or(DEFAULT_GLOBAL_CONFIGURATION.indent_width),
                &mut diagnostics,
            ),
        };

        diagnostics.extend(get_unknown_property_diagnostics(config));

        ResolveConfigurationResult {
            config: resolved_config,
            diagnostics,
        }
    }
}
