use ext_php_rs::prelude::*;
use ext_php_rs::exception::PhpException;
use pagefind::options::PagefindServiceConfig;
use pagefind::api::{PagefindIndex};

use crate::exception::PagefindException;
use crate::service_config::PhpPagefindServiceConfig;
use crate::response::PhpPagefindResponse;
use crate::get_runtime;

#[php_class]
#[php(name = "Pagefind\\Indexer")]
pub struct PhpPagefindIndex {
    inner: PagefindIndex,
}

#[php_impl]
#[php(change_method_case = "camelCase")]
impl PhpPagefindIndex {
    pub fn __construct(config: &PhpPagefindServiceConfig) -> Result<Self, PhpException> {
        // Create a new PagefindServiceConfig with the same settings
        let service_config = PagefindServiceConfig::builder()
            .keep_index_url(config.keep_url)
            .force_language(config.fallback_language.to_string())
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
