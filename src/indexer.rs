use ext_php_rs::binary::Binary;
use ext_php_rs::exception::PhpException;
use ext_php_rs::prelude::*;
use pagefind::api::PagefindIndex;
use pagefind::options::PagefindServiceConfig;

use crate::exception::PagefindException;
use crate::file::PhpPagefindFile;
use crate::get_runtime;
use crate::response::PhpPagefindResponse;
use crate::service_config::PhpPagefindServiceConfig;

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
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!(
                "Failed to create PagefindIndex: {}",
                e
            ))),
        }
    }

    pub fn add_html_file(
        &mut self,
        source_path: String,
        url: String,
        content: String,
    ) -> Result<PhpPagefindResponse, PhpException> {
        // Use Tokio runtime to handle the async function call
        let response = get_runtime().block_on(self.inner.add_html_file(
            Some(source_path.clone()),
            Some(url.clone()),
            content,
        ));

        match response {
            Ok(metadata) => Ok(PhpPagefindResponse {
                success: true,
                message: format!("Successfully added HTML file: {}", source_path),
                metadata: Some(format!("{:?}", metadata)),
            }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!(
                "Error adding HTML file: {}",
                e
            ))),
        }
    }

    pub fn add_directory(
        &mut self,
        path: String,
        pattern: Option<String>,
    ) -> Result<PhpPagefindResponse, PhpException> {
        let response =
            get_runtime().block_on(self.inner.add_directory(path.clone(), pattern.clone()));

        match response {
            Ok(metadata) => Ok(PhpPagefindResponse {
                success: true,
                message: format!("Successfully added directory: {}", path),
                metadata: Some(format!("{:?}", metadata)),
            }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!(
                "Error adding directory: {}",
                e
            ))),
        }
    }

    pub fn write_files(
        &mut self,
        output_directory: String,
    ) -> Result<PhpPagefindResponse, PhpException> {
        let response =
            get_runtime().block_on(self.inner.write_files(Some(output_directory.clone())));

        match response {
            Ok(metadata) => Ok(PhpPagefindResponse {
                success: true,
                message: format!("Successfully wrote files to: {}", output_directory),
                metadata: Some(format!("{:?}", metadata)),
            }),
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!(
                "Error writing files: {}",
                e
            ))),
        }
    }

    pub fn get_files(&mut self) -> Result<Vec<PhpPagefindFile>, PhpException> {
        match get_runtime().block_on(self.inner.get_files()) {
            Ok(files) => {
                let mut result = Vec::new();

                for file in files {
                    // Convert PathBuf to String, handling potential conversion errors
                    let filename = match file.filename.to_str() {
                        Some(s) => s.to_string(),
                        None => {
                            return Err(PhpException::from_class::<PagefindException>(
                                "Invalid UTF-8 in filename".to_string(),
                            ))
                        }
                    };

                    result.push(PhpPagefindFile {
                        filename: filename,
                        contents: Binary::from(file.contents),
                    });
                }

                Ok(result)
            }
            Err(e) => Err(PhpException::from_class::<PagefindException>(format!(
                "Error getting files: {}",
                e
            ))),
        }
    }
}
