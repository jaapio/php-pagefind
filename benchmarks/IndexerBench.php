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
    private array $sampleContent = [];

    public function setUp(): void
    {
        $this->sampleDir = __DIR__ . '/../test/html_samples';
        $this->outputDir = '/tmp/pagefind-benchmark-' . uniqid();

        if (!is_dir($this->outputDir)) {
            mkdir($this->outputDir, 0777, true);
        }

        $this->config = new ServiceConfig(false, false);

        // Cache sample content to avoid reading from disk during benchmarks
        $this->sampleContent = [
            'article1' => file_get_contents($this->sampleDir . '/article1.html'),
            'article2' => file_get_contents($this->sampleDir . '/article2.html'),
            'article3' => file_get_contents($this->sampleDir . '/article3.html')
        ];
    }

    /**
     * @Revs(5)
     * @Iterations(5)
     */
    public function benchIndexEmpty(): void
    {
        $indexer = new Indexer($this->config);
        $files = $indexer->writeFiles($this->outputDir);
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
            $this->sampleContent['article1']
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
            $this->sampleContent['article1']
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
            $this->sampleContent['article1']
        );

        $indexer->addHtmlFile(
            'article2',
            '/article2',
            $this->sampleContent['article2']
        );

        $indexer->addHtmlFile(
            'article3',
            '/article3',
            $this->sampleContent['article3']
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
            $this->sampleContent['article1']
        );

        $indexer->addHtmlFile(
            'article2',
            '/article2',
            $this->sampleContent['article2']
        );

        $indexer->addHtmlFile(
            'article3',
            '/article3',
            $this->sampleContent['article3']
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

    /**
     * Benchmark the impact of increasing file count
     *
     * @Revs(1)
     * @Iterations(1)
     * @ParamProviders("provideFileCounts")
     */
    public function benchScalingFileCount(array $params): void
    {
        $fileCount = $params['count'];
        $indexer = new Indexer($this->config);

        // Generate and add the specified number of files
        for ($i = 0; $i < $fileCount; $i++) {
            // Rotate through our sample content to create varied content
            $contentKey = array_keys($this->sampleContent)[$i % count($this->sampleContent)];

            $indexer->addHtmlFile(
                "page-{$i}",
                "/page-{$i}",
                $this->sampleContent[$contentKey]
            );
        }

        $indexer->writeFiles($this->outputDir);
    }

    /**
     * Provides parameters for scaling benchmarks
     */
    public function provideFileCounts(): \Generator
    {
        yield 'files-10' => ['count' => 10];
        yield 'files-25' => ['count' => 25];
        yield 'files-50' => ['count' => 50];
        yield 'files-100' => ['count' => 100];
        yield 'files-10000' => ['count' => 10000];
    }
}
