pub mod configuration;
pub(crate) mod parser;
pub(crate) mod plugin;

use crate::configuration::Configuration;
use dprint_core::plugins::PluginHandler;
use plugin::VuePluginHandler;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(VuePluginHandler, VuePluginHandler::new());

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use dprint_core::plugins::PluginHandler;

    use crate::{configuration::Configuration, plugin::VuePluginHandler};

    #[test]
    fn test_format_file() {
        let mut buffer = Vec::new();

        let raw = include_str!("../test/file.vue");
        let path = PathBuf::from("ts.vue");

        let result = VuePluginHandler::new().format_text(
            &path,
            raw,
            &Configuration::default(),
            |path, data, _config| {
                buffer.push((path.to_owned(), data.clone()));

                Ok(data)
            },
        );

        assert_eq!(result.unwrap(), raw);

        assert_eq!(
            buffer,
            vec![
                (
                    PathBuf::from("file.html"),
                    String::from("    <template>\n        <template></template>\n        <template></template>\n    </template>\n    <template></template>\n    <template></template>\n")
                ),
                (
                    PathBuf::from("file.ts"),
                    String::from("import { ExclamationIcon } from '@heroicons/vue/solid';\nimport { Dialog, DialogOverlay, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';\nimport { watch } from \"vue\";\n\nimport type { NewQuoteRequest, QuoteRequest, QuoteRequestId, QuoteRequestPatch } from \"@/resources/quoteRequests\";\nimport type { ApiError, AbortError, NetworkError } from \"@/error\";\nimport { usePassport } from \"@/passport\";\nimport { createQuoteRequest, retrieveQuoteRequest, updateQuoteRequest, destroyQuoteRequest } from \"@/resources/quoteRequests\";\n\n"),
                )
            ]
        );
    }
}
