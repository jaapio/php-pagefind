<?php

// Create the configuration with keep_url=true and verbose=true for more output
$config = new Pagefind\ServiceConfig(true, true);

// Create the indexer with our configuration
$indexer = new Pagefind\Indexer($config);

echo "=== Testing Individual File Indexing ===\n";
echo "Adding individual HTML file: ";
$result1 = $indexer->add_html_file(
    'individual-document',
    '/individual-page',
    '<h1>Test Page</h1><p>This is a simple test document for individual indexing.</p>'
);
var_dump($result1);

echo "\n=== Testing Directory Indexing ===\n";
// Get the absolute path to the html_samples directory
$samplesDir = __DIR__ . '/html_samples';
echo "Indexing all HTML files in: $samplesDir\n";

// Add all HTML files from the directory
$directoryResult = $indexer->add_directory($samplesDir);
echo "Directory indexing result: ";
var_dump($directoryResult);

// Write the index files to the specified directory
echo "\n=== Generating Search Index ===\n";
$outputDir = dirname(__DIR__) . '/build/pagefind';
echo "Writing index files to: $outputDir\n";
$writeResult = $indexer->write_files($outputDir);
echo "Index generation result: ";
var_dump($writeResult);

echo "\nIndex generation " . ($writeResult ? "completed successfully" : "failed") . ".\n";
if ($writeResult) {
    echo "Search index files were written to: $outputDir\n";
    echo "\nYou can now integrate the search functionality into your website by adding:\n";
    echo "1. Include the PageFind JS file\n";
    echo "2. Add search UI elements\n";
    echo "3. Initialize the PageFind search\n";
}
