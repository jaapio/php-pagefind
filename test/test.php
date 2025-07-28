<?php

$config = new Pagefind\ServiceConfig(true, true);

$indexer = new Pagefind\Indexer($config);
echo "Adding HTML file: ";
var_dump($indexer->add_html_file('test/test.html', '/test', '<html><body><h1>Test content</h1></body></html>'));

// Add more test documents if needed
$indexer->add_html_file('test/another.html', '/another', '<html><body><h1>Test other content</h1></body></html>');

// Write the index files to the pagefind_index directory
echo "Writing index files: ";
var_dump($indexer->write_files('./build/pagefind'));

echo "Index files written to ./build/pagefind\n";
