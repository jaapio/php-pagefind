# Use the official Rust nightly image as the build environment
FROM rustlang/rust:nightly AS builder

# Install PHP development headers and Clang for bindgen
RUN apt-get update && \
    apt-get install -y php-dev clang libclang-dev llvm-dev && \
    rm -rf /var/lib/apt/lists/*

# Set environment variable for bindgen to find libclang
ENV LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu

WORKDIR /app

# Copy source code and manifest
COPY . .

# Build the project in release mode
RUN cargo build --release

# Create a minimal image with only the compiled library
FROM php:8.2-cli
WORKDIR /app

# Determine PHP extension directory in the new image
RUN mkdir -p /usr/local/lib/php/extensions && \
    php -i | grep "extension_dir => /" | awk '{print $3}' > /extension_dir.txt

# Copy the compiled library from the builder
COPY --from=builder /app/target/release/libphp_pagefind.so /tmp/
# Move to the correct directory using the path we saved
RUN mkdir -p $(cat /extension_dir.txt) && \
    cp /tmp/libphp_pagefind.so $(cat /extension_dir.txt)/php_pagefind.so && \
    rm /tmp/libphp_pagefind.so /extension_dir.txt

# Enable the extension by creating a config file
RUN echo "extension=php_pagefind" > /usr/local/etc/php/conf.d/php_pagefind.ini

# Set default command
CMD ["php", "-m"]
