# Build stage
FROM lewimbes/dioxus:0.6.3 AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# run tailwind
RUN npx tailwindcss -i ./input.css -o ./assets/tailwind.css

# build app
RUN dx build --release --platform web

# Final stage
FROM debian:bookworm-slim

# Install necessary dependencies for running the server
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the entire web directory from the builder stage
COPY --from=builder /usr/src/app/target/dx/rich_darts/release/web ./

COPY migrations /app/migrations
ENV MIGRATION_URI="/app/migrations"

ENV PORT=8080
ENV IP=0.0.0.0
# Expose the port your server listens on (adjust if necessary)
EXPOSE 8080

# Run the server
CMD ["./server"]
