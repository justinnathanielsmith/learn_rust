# Use the official Rust image
FROM rust:1.94

# Set the working directory inside the container
WORKDIR /app

# Copy your source code
COPY . .

# Keep the container running so we can exec into it
CMD ["tail", "-f", "/dev/null"]