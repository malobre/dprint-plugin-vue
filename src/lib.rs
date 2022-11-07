mod configuration;
mod format;
mod plugin;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm {
    use crate::{configuration::Configuration, plugin::VuePluginHandler};
    use dprint_core::plugins::SyncPluginHandler;
    dprint_core::generate_plugin_code!(VuePluginHandler, VuePluginHandler::new());
}
