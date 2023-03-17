FROM debian AS builder
WORKDIR /code
ENV PATH="/root/.cargo/bin:${PATH}"
RUN apt update \
    && apt install curl -y \
    && curl https://sh.rustup.rs -sSf | bash -s -- -y \
    && apt-get update \
    && apt-get install -y \
        build-essential \
        libunwind-dev \
        libssl-dev \
        lldb \
        pkg-config \
        binutils-dev \
    && apt-get clean \
    && apt install git -y \
    && cargo install honggfuzz \
    && cargo install cargo-fuzz \
    && rustup toolchain install nightly

RUN git clone https://github.com/lambdaclass/fuzzing_examples.git
