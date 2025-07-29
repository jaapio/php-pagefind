use ext_php_rs::prelude::*;

#[php_class]
#[php(name = "Pagefind\\Response")]
pub struct PhpPagefindResponse {
    pub success: bool,
    pub message: String,
    pub metadata: Option<String>,
}

#[php_impl]
#[php(change_method_case = "camelCase")]
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
