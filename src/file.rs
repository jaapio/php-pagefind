use ext_php_rs::binary::Binary;
// File: file.rs - File class definition
use ext_php_rs::prelude::*;

#[php_class]
#[php(name = "Pagefind\\File")]
pub struct PhpPagefindFile {
    pub filename: String,
    pub contents: Binary<u8>,
}

#[php_impl]
#[php(change_method_case = "camelCase")]
impl PhpPagefindFile {
    /// Create a new File instance
    pub fn __construct(filename: String, contents: Binary<u8>) -> Self {
        Self { filename, contents }
    }

    /// Get the filename
    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    /// Get the contents
    pub fn get_contents(&self) -> Binary<u8> {
        self.contents.clone().into()
    }
}
