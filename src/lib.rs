#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ext_php_rs::{
    info_table_end, info_table_row, info_table_start, zend::ModuleEntry,
};
use tokio::runtime::Runtime;

// Module declarations
pub mod exception;
pub mod service_config;
pub mod response;
pub mod indexer;

// Import the classes we need for module registration
use exception::PagefindException;
use service_config::PhpPagefindServiceConfig;
use response::PhpPagefindResponse;
use indexer::PhpPagefindIndex;

// Global runtime for async operations
pub fn get_runtime() -> &'static Runtime {
    static mut RUNTIME: Option<Runtime> = None;
    static INIT: std::sync::Once = std::sync::Once::new();

    unsafe {
        INIT.call_once(|| {
            RUNTIME = Some(Runtime::new().expect("Failed to create Tokio runtime"));
        });
        RUNTIME.as_ref().unwrap()
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
