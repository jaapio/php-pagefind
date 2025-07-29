# Use the official Rust nightly image as the build environment
FROM rustlang/rust:nightly AS builder

# Install PHP development headers and Clang for bindgen
RUN apt-get update && \
    apt-get install -y php-dev clang libclang-dev llvm-dev && \
    rm -rf /var/lib/apt/lists/*

# Set environment variable for bindgen to find libclang
ENV LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu

WORKDIR /app

# Copy only Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock* ./

# Create a dummy src/lib.rs file to compile dependencies
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy implementation"); }' > src/lib.rs && \
    cargo build --release || true && \
    rm -rf src

# Now copy the real source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# Create a minimal image with only the compiled library
FROM php:8.2-cli
WORKDIR /app

# Install zip and unzip for Composer
RUN apt-get update && \
    apt-get install -y zip unzip git && \
    rm -rf /var/lib/apt/lists/*

# Install Composer
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer

# Determine PHP extension directory in the new image
RUN mkdir -p /usr/local/lib/php/extensions && \
    php -i | grep "extension_dir => /" | awk '{print $3}' > /extension_dir.txt

# Copy the compiled library from the builder
COPY --from=builder /app/target/release/libpagefind.so /tmp/
# Move to the correct directory using the path we saved
RUN mkdir -p $(cat /extension_dir.txt) && \
    cp /tmp/libpagefind.so $(cat /extension_dir.txt)/pagefind.so && \
    rm /tmp/libpagefind.so /extension_dir.txt

# Enable the extension by creating a config file
RUN echo "extension=pagefind" > /usr/local/etc/php/conf.d/pagefind.ini

# Set default command
CMD ["php", "-m"]
