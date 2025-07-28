#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ext_php_rs::{
    info_table_end, info_table_row, info_table_start, zend::ModuleEntry, exception::PhpException,
    zend::ce,
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
#[php(name = "Pagefind\\Exception")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct PagefindException;

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
#[php(name = "Pagefind\\Response")]
pub struct PhpPagefindResponse {
    success: bool,
    message: String,
    metadata: Option<String>,
}

#[php_impl]
#[php(change_method_case = "snake_case")]
impl PhpPagefindResponse {
    /// Create a new Response
    pub fn __construct(success: bool, message: String, metadata: Option<String>) -> Self {
        Self {
            success,
            message,
            metadata,
        }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_metadata(&self) -> Option<String> {
        self.metadata.clone()
    }
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
    pub fn __construct(config: &PhpPagefindServiceConfig) -> Result<Self, PhpException> {
        // Create a new PagefindServiceConfig with the same settings
        let service_config = PagefindServiceConfig::builder()
            .keep_index_url(config.keep_url)
            .force_language("en".to_string())
            .verbose(config.verbose)
            .build();

        match PagefindIndex::new(Some(service_config)) {
            Ok(index) => Ok(Self { inner: index }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!("Failed to create PagefindIndex: {}", e))),
        }
    }

    pub fn add_html_file(&mut self, source_path: String, url: String, content: String) -> Result<PhpPagefindResponse, PhpException> {
        // Use Tokio runtime to handle the async function call
        let response = get_runtime().block_on(self.inner.add_html_file(
            Some(source_path.clone()),
            Some(url.clone()),
            content
        ));

        match response {
            Ok(metadata) => Ok(PhpPagefindResponse {
                success: true,
                message: format!("Successfully added HTML file: {}", source_path),
                metadata: Some(format!("{:?}", metadata)),
            }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!("Error adding HTML file: {}", e))),
        }
    }

    pub fn add_directory(&mut self, path: String, pattern: Option<String>) -> Result<PhpPagefindResponse, PhpException> {
        let response = get_runtime().block_on(
                self.inner.add_directory(path.clone(), pattern.clone())
        );

        match response {
            Ok(metadata) => Ok(PhpPagefindResponse {
                success: true,
                message: format!("Successfully added directory: {}", path),
                metadata: Some(format!("{:?}", metadata)),
            }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!("Error adding directory: {}", e))),
        }
    }

    pub fn write_files(&mut self, output_directory: String) -> Result<PhpPagefindResponse, PhpException> {
        let response = get_runtime().block_on(self.inner.write_files(
            Some(output_directory.clone())
        ));

        match response {
            Ok(metadata) => Ok(PhpPagefindResponse {
                success: true,
                message: format!("Successfully wrote files to: {}", output_directory),
                metadata: Some(format!("{:?}", metadata)),
            }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!("Error writing files: {}", e))),
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
        .class::<PhpPagefindResponse>()
        .class::<PhpPagefindIndex>()
        .class::<PagefindException>()
        .info_function(php_module_info)
}
