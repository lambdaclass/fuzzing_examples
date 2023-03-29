FROM --platform=linux/amd64 debian AS builder
WORKDIR /code
RUN apt update -y
RUN apt install -y curl \
                   build-essential \
                   libunwind-dev \
                   libssl-dev \
                   lldb \
                   pkg-config \
                   binutils-dev \
                   git
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install honggfuzz
RUN cargo install cargo-fuzz

RUN git clone https://github.com/lambdaclass/fuzzing_examples.git
