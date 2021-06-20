from rust:latest as builder
workdir /usr/src/tilt
copy . .
run cargo install --path .

from debian:buster-slim
run apt-get update && apt-get install -y bluez && rm -rf /var/lib/apt/lists/*
copy --from=builder /usr/local/cargo/bin/tilt /usr/local/bin/tilt
cmd ["/bin/sh", "-c",  "tilt 2>/dev/null"]
