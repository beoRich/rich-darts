# Build stage
FROM rust:1-alpine3.21 AS builder

RUN apk add --no-cache build-base musl-dev openssl-dev openssl

# Install dioxus-cli
RUN cargo install cargo-binstall
RUN cargo binstall dioxus-cli


# tailwind + daisyUi
RUN apk update && apk --no-cache add nodejs npm
RUN npm install tailwindcss @tailwindcss/cli
RUN npm i -D daisyui@latest
RUN npx tailwindcss -i ./input.css -o ./assets/tailwind.css

# build app
RUN dx build --release --platform web

# Final stage
FROM alpine:3.21 

WORKDIR /app

# Copy the entire web directory from the builder stage
COPY --from=builder /usr/src/app/target/dx/DrawsNotes/release/web ./

ENV PORT=8080
ENV IP=0.0.0.0
# Expose the port your server listens on (adjust if necessary)
EXPOSE 8080

# Run the server
CMD ["./server"]
