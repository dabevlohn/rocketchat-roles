FROM rust:1.74 as build
# FROM rust:1-alpine3.19

ENV RUSTFLAGS="-C target-feature=-crt-static"

# create a new empty shell project
RUN USER=root cargo new --bin rocketmongoman
WORKDIR /rocketmongoman

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml

# this build step will cache your dependencies
# RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release
RUN cargo install --path .
RUN rm ./target/release/deps/*

# our final base
# FROM debian:buster-slim
# FROM alpine:3.19
# FROM rust:1.74-slim-buster

# copy the build artifact from the build stage
# COPY --from=build /rocketmongoman/target/release/rocketmongoman .

# set the startup command to run your binary
# CMD ["./rocketmongoman"]
CMD ["rocketmongoman"]
