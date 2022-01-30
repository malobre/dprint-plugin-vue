mod configuration;
mod format;
mod parser;
mod plugin;

#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm"))]
mod wasm {
    // Ignore warnings generated by the macro.
    #![allow(unused_must_use)]
    #![allow(clippy::mut_from_ref)]

    use crate::{configuration::Configuration, plugin::VuePluginHandler};
    use dprint_core::plugins::PluginHandler;
    dprint_core::generate_plugin_code!(VuePluginHandler, VuePluginHandler::new());
}
