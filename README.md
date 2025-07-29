# PHP PageFind Extension

A PHP extension for integrating the Rust-based PageFind search indexing library into PHP applications. PageFind is a static search library that works without a backend, making it ideal for static sites and JAMstack applications.

## Overview

This PHP extension wraps the PageFind Rust library, allowing you to:

- Index HTML content directly from PHP
- Add entire directories of HTML files to the search index
- Write search index files that can be served statically

The extension handles all the interaction with the PageFind Rust library, providing a clean PHP API that follows PHP's best practices for error handling and response management.

## Installation

*Prerequisites: Rust toolchain must be installed.*

```bash
git clone https://github.com/jaapio/php-pagefind.git
cd php-pagefind
cargo build
# Copy the resulting .so file to your PHP extensions directory
```

## Usage

### Basic Usage

```php
<?php
// Create a configuration
$config = new Pagefind\ServiceConfig(
    true,  // keep URL
    true,  // verbose mode
    'en'   // fallback language (optional, defaults to 'en')
);

try {
    // Initialize the indexer
    $indexer = new Pagefind\Indexer($config);
    
    // Index an HTML string
    $response = $indexer->addHtmlFile(
        'document-id',
        '/page-url',
        '<h1>Page Title</h1><p>Page content for indexing</p>'
    );
    
    // Index a directory of HTML files (with optional file pattern)
    $response = $indexer->addDirectory('/path/to/html/files', '*.html');
    
    // Write the search index to a directory
    $response = $indexer->writeFiles('/path/to/output/directory');
    
    echo "Search index created successfully: " . $response->getMessage();
    
} catch (Pagefind\Exception $e) {
    echo "Error: " . $e->getMessage();
}
```

### Response Object

All successful operations return a `Pagefind\Response` object with:

```php
// Check if the operation was successful
$success = $response->isSuccess();

// Get a descriptive message about the operation
$message = $response->getMessage();

// Get any metadata from the operation (returns null if no metadata)
$metadata = $response->getMetadata();
```

### Exception Handling

The extension uses exceptions for error handling. All errors throw a `Pagefind\Exception` that extends PHP's native Exception class:

```php
try {
    $indexer->addDirectory('/non/existent/path');
} catch (Pagefind\Exception $e) {
    // Handle the error
    echo "Error indexing directory: " . $e->getMessage();
}
```

## API Reference

### Pagefind\ServiceConfig

Configuration options for the PageFind service.

```php
__construct(bool $keep_url, bool $verbose, string $fallback_language = 'en')
```

- `$keep_url`: Whether to keep the original URL in the index
- `$verbose`: Whether to output verbose information
- `$fallback_language`: The language to use when no language is specified (default: 'en')

### Pagefind\Indexer

The main class for indexing content.

```php
// Initialize with a configuration
__construct(Pagefind\ServiceConfig $config)

// Add an HTML file to the index
add_html_file(string $source_path, string $url, string $content): Pagefind\Response

// Add a directory of HTML files to the index
add_directory(string $path, ?string $pattern = null): Pagefind\Response

// Write the index files to the specified directory
write_files(string $output_directory): Pagefind\Response
```

### Pagefind\Response

Response object returned by successful operations.

```php
is_success(): bool
get_message(): string
get_metadata(): ?string
```

## License

[MIT License](LICENSE)

## Credits

- Built on [PageFind](https://pagefind.app/)
- PHP integration by [Jaapio](https://github.com/jaapio)
