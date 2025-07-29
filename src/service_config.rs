use ext_php_rs::prelude::*;

#[php_class]
#[php(name = "Pagefind\\ServiceConfig")]
pub struct PhpPagefindServiceConfig {
    pub keep_url: bool,
    pub verbose: bool,
    pub fallback_language: String,
}

#[php_impl()]
impl PhpPagefindServiceConfig {
    pub fn __construct(
        keep_url: bool,
        verbose: bool,
        fallback_language: Option<String>,
    ) -> Self {
        Self {
            keep_url,
            verbose,
            fallback_language: fallback_language.unwrap_or_else(|| "en".to_string()),
        }
    }
}
