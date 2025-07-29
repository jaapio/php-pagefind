<?php

namespace PhpPagefind\Benchmarks;

use Pagefind\Indexer;
use Pagefind\ServiceConfig;

/**
 * @BeforeMethods({"setUp"})
 */
class IndexerBench
{
    private string $sampleDir;
    private ServiceConfig $config;
    private string $outputDir;

    public function setUp(): void
    {
        $this->sampleDir = __DIR__ . '/../test/html_samples';
        $this->outputDir = '/tmp/pagefind-benchmark-' . uniqid();

        if (!is_dir($this->outputDir)) {
            mkdir($this->outputDir, 0777, true);
        }

        $this->config = new ServiceConfig(false, false);
    }

    /**
     * @Revs(5)
     * @Iterations(5)
     */
    public function benchIndexSinglePage(): void
    {
        $indexer = new Indexer($this->config);

        $indexer->addHtmlFile(
            'article1',
            '/article1',
            file_get_contents($this->sampleDir . '/article1.html')
        );

        $files = $indexer->getFiles();

        foreach ($files as $file) {
            $filePath = $this->outputDir . '/' . $file->getFileName();
            $dirPath = dirname($filePath);

            if (!is_dir($dirPath)) {
                mkdir($dirPath, 0777, true);
            }

            file_put_contents($filePath, $file->getContents());
        }
    }

    /**
     * @Revs(5)
     * @Iterations(5)
     */
    public function benchIndexSinglePageWithWriteFiles(): void
    {
        $indexer = new Indexer($this->config);

        $indexer->addHtmlFile(
            'article1',
            '/article1',
            file_get_contents($this->sampleDir . '/article1.html')
        );

        $indexer->writeFiles($this->outputDir);
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchIndexMultiplePages(): void
    {
        $indexer = new Indexer($this->config);

        $indexer->addHtmlFile(
            'article1',
            '/article1',
            file_get_contents($this->sampleDir . '/article1.html')
        );

        $indexer->addHtmlFile(
            'article2',
            '/article2',
            file_get_contents($this->sampleDir . '/article2.html')
        );

        $indexer->addHtmlFile(
            'article3',
            '/article3',
            file_get_contents($this->sampleDir . '/article3.html')
        );

        $files = $indexer->getFiles();

        foreach ($files as $file) {
            $filePath = $this->outputDir . '/' . $file->getFileName();
            $dirPath = dirname($filePath);

            if (!is_dir($dirPath)) {
                mkdir($dirPath, 0777, true);
            }

            file_put_contents($filePath, $file->getContents());
        }
    }

    /**
     * @Revs(3)
     * @Iterations(3)
     */
    public function benchIndexMultiplePagesWithWriteFiles(): void
    {
        $indexer = new Indexer($this->config);

        $indexer->addHtmlFile(
            'article1',
            '/article1',
            file_get_contents($this->sampleDir . '/article1.html')
        );

        $indexer->addHtmlFile(
            'article2',
            '/article2',
            file_get_contents($this->sampleDir . '/article2.html')
        );

        $indexer->addHtmlFile(
            'article3',
            '/article3',
            file_get_contents($this->sampleDir . '/article3.html')
        );

        $indexer->writeFiles($this->outputDir);
    }

    /**
     * @Revs(2)
     * @Iterations(2)
     */
    public function benchIndexDirectory(): void
    {
        $indexer = new Indexer($this->config);

        $indexer->addDirectory($this->sampleDir);

        $files = $indexer->getFiles();

        foreach ($files as $file) {
            $filePath = $this->outputDir . '/' . $file->getFileName();
            $dirPath = dirname($filePath);

            if (!is_dir($dirPath)) {
                mkdir($dirPath, 0777, true);
            }

            file_put_contents($filePath, $file->getContents());
        }
    }

    /**
     * @Revs(2)
     * @Iterations(2)
     */
    public function benchIndexDirectoryWithWriteFiles(): void
    {
        $indexer = new Indexer($this->config);

        $indexer->addDirectory($this->sampleDir);

        $indexer->writeFiles($this->outputDir);
    }
}
