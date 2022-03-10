FROM rust:1.59.0-alpine3.15 as builder
WORKDIR /app
COPY . .
RUN ["apk", "add", "--no-cache", "build-base"]
RUN ["cargo", "build", "--release"]

FROM alpine:3.15
COPY --from=builder /app/target/release/vulpine /usr/local/bin
CMD ["vulpine"]
