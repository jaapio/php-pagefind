<?php

// Create the configuration with keep_url=true and verbose=true for more output
$config = new Pagefind\ServiceConfig(true, true);

try {
    // Create the indexer with our configuration
    $indexer = new Pagefind\Indexer($config);

    echo "=== Testing Individual File Indexing ===\n";
    echo "Adding individual HTML file: \n";
    try {
        $response = $indexer->add_html_file(
            'individual-document',
            '/individual-page',
            '<h1>Test Page</h1><p>This is a simple test document for individual indexing.</p>'
        );

        echo "Success: " . ($response->is_success() ? "Yes" : "No") . "\n";
        echo "Message: " . $response->get_message() . "\n";
        if ($metadata = $response->get_metadata()) {
            echo "Metadata: " . $metadata . "\n";
        }
    } catch (Pagefind\Exception $e) {
        echo "Error adding HTML file: " . $e->getMessage() . "\n";
    }

    echo "\n=== Testing Directory Indexing ===\n";
    // Get the absolute path to the html_samples directory
    $samplesDir = __DIR__ . '/html_samples';
    echo "Indexing all HTML files in: $samplesDir\n";

    // Add all HTML files from the directory
    try {
        $response = $indexer->add_directory($samplesDir);
        echo "Success: " . ($response->is_success() ? "Yes" : "No") . "\n";
        echo "Message: " . $response->get_message() . "\n";
        if ($metadata = $response->get_metadata()) {
            echo "Metadata: " . $metadata . "\n";
        }
    } catch (Pagefind\Exception $e) {
        echo "Error indexing directory: " . $e->getMessage() . "\n";
    }

    // Write the index files to the specified directory
    echo "\n=== Generating Search Index ===\n";
    $outputDir = dirname(__DIR__) . '/build/pagefind';
    echo "Writing index files to: $outputDir\n";

    try {
        $response = $indexer->write_files($outputDir);
        echo "Success: " . ($response->is_success() ? "Yes" : "No") . "\n";
        echo "Message: " . $response->get_message() . "\n";
        if ($metadata = $response->get_metadata()) {
            echo "Metadata: " . $metadata . "\n";
        }

        echo "\nIndex generation completed successfully.\n";
        echo "Search index files were written to: $outputDir\n";
        echo "\nYou can now integrate the search functionality into your website by adding:\n";
        echo "1. Include the PageFind JS file\n";
        echo "2. Add search UI elements\n";
        echo "3. Initialize the PageFind search\n";
    } catch (Pagefind\Exception $e) {
        echo "Error generating index: " . $e->getMessage() . "\n";
    }
} catch (Pagefind\Exception $e) {
    echo "Error initializing PageFind indexer: " . $e->getMessage() . "\n";
}
