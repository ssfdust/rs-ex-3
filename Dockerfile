FROM rust:latest

RUN mkdir /root/build /root/.cargo
COPY config /root/.cargo/config
COPY . /root/build
WORKDIR /root/build
RUN echo deb http://deb.debian.org/debian bullseye devel >> /etc/apt/sources.list \
    && apt update \
    && apt install -y mingw-w64
RUN cd /root/build \
    && rustup target add x86_64-pc-windows-gnu \
    && rustup toolchain install stable-x86_64-pc-windows-gnu \
    && cargo build --target x86_64-pc-windows-gnu --release \
    && cargo build --release
