#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ext_php_rs::{
    info_table_end, info_table_row, info_table_start, zend::ModuleEntry,
};
use pagefind::options::PagefindServiceConfig;
use pagefind::api::{PagefindIndex};
use tokio::runtime::Runtime;

// Global runtime for async operations
fn get_runtime() -> &'static Runtime {
    static mut RUNTIME: Option<Runtime> = None;
    static INIT: std::sync::Once = std::sync::Once::new();

    unsafe {
        INIT.call_once(|| {
            RUNTIME = Some(Runtime::new().expect("Failed to create Tokio runtime"));
        });
        RUNTIME.as_ref().unwrap()
    }
}

#[php_class]
#[php(name = "Pagefind\\ServiceConfig")]
pub struct PhpPagefindServiceConfig {
    keep_url: bool,
    verbose: bool
}

#[php_impl()]
impl PhpPagefindServiceConfig {
    pub fn __construct(
        keep_url: bool,
        verbose: bool,
    ) -> Self {
        Self {
            keep_url,
            verbose
        }
    }

    // Add methods to set config fields as needed
}

#[php_class]
#[php(name = "Pagefind\\Indexer")]
pub struct PhpPagefindIndex {
    inner: PagefindIndex,
}

#[php_impl]
#[php(change_method_case = "snake_case")]
impl PhpPagefindIndex {
    /// Create a new PagefindIndex
    pub fn __construct(config: &PhpPagefindServiceConfig) -> Self {
        // Create a new PagefindServiceConfig with the same settings
        let service_config = PagefindServiceConfig::builder()
            .keep_index_url(config.keep_url)
            .force_language("en".to_string())
            .verbose(config.verbose)
            .build();

        Self {
            inner: PagefindIndex::new(Some(service_config)).expect("Options to be valid")
        }
    }

    pub fn add_html_file(&mut self, source_path: String, url: String, content: String) -> bool {
        // Use Tokio runtime to handle the async function call
        let response = get_runtime().block_on(self.inner.add_html_file(
            Some(source_path),
            Some(url),
            content
        ));

        match response {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error adding HTML file: {}", e);
                false
            }
        }
    }

    pub fn add_directory(&mut self, path: String, pattern: Option<String>) -> bool {
        let response = get_runtime().block_on(
                self.inner.add_directory(path, pattern)
        );

        match response {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error adding directory: {}", e);
                false
            }
        }
    }

    pub fn write_files(&mut self, output_directory: String) -> bool {
        let response = get_runtime().block_on(self.inner.write_files(
            Some(output_directory)
        ));

        match response {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error writing files: {}", e);
                false
            }
        }
    }
}

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("PageFind", "enabled");
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<PhpPagefindServiceConfig>()
        .class::<PhpPagefindIndex>()
        .info_function(php_module_info)
}