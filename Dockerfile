# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build the application
RUN cargo build --release --features arkworks

# Use the official Golang image to download the binary
FROM golang:latest AS golang_builder

# Set the working directory inside the container
WORKDIR /app

# Download the golang binary
RUN wget https://github.com/drand/drand/releases/download/v1.5.8-testnest/drand-v1.5.8-testnest-linux-amd64.tar.gz \
    && tar -xzf drand-v1.5.8-testnest-linux-amd64.tar.gz \
    && mv drand /app/make_demo/drand_go

# Use a minimal base image for the final stage
FROM debian:bullseye-slim   

# Set the working directory
WORKDIR /app

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/drand-rs /app/make_demo/drand_rs

# Copy the Golang binary from the golang_builder stage
COPY --from=golang_builder /app/make_demo/drand_go /app/make_demo/drand_go

# Copy the run script
COPY run /app/run

# Make the run script executable
RUN chmod +x /app/run

# Set the entrypoint to the run script
ENTRYPOINT ["./run"]