# We use the official Rust image (Stable)
FROM rust:1.94

# Install system dependencies often needed for building Rust crates
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for development
ARG USERNAME=rust
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && chown -R $USERNAME:$USERNAME /usr/local/cargo

USER $USERNAME