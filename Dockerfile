# Use the official Ubuntu base image
FROM ubuntu:latest

# Set the environment variable to non-interactive mode for apt-get
ENV DEBIAN_FRONTEND=noninteractive

# Copy the local repository into the container
COPY . /workspace

# Set the working directory
WORKDIR /workspace

# Update the package list and install necessary packages
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    git \
    && apt-get clean

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the path for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build