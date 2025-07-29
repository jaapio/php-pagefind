.PHONY: build benchmark benchmark-html clean setup

# Docker image name
IMAGE_NAME = php-pagefind
CONTAINER_NAME = php-pagefind-benchmark

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Setup dependencies
setup: build
	docker run --rm -v $(PWD):/app --name $(CONTAINER_NAME)-setup $(IMAGE_NAME) bash -c "cd /app && git config --global --add safe.directory /app && composer install"

# Run phpbench
benchmark:
	docker run --rm -v $(PWD):/app --name $(CONTAINER_NAME) $(IMAGE_NAME) bash -c "cd /app && git config --global --add safe.directory /app && vendor/bin/phpbench run --report=default"

# Generate HTML report
benchmark-html: setup
	docker run --rm -v $(PWD):/app --name $(CONTAINER_NAME) $(IMAGE_NAME) bash -c "cd /app && git config --global --add safe.directory /app && vendor/bin/phpbench run --report=default --output=html"

# Clean up
clean:
	rm -rf build/benchmark_out
	rm -rf vendor
	rm -f benchmarks/report.html
