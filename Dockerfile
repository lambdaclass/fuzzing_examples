# first construction stage
FROM debian AS builder
WORKDIR /code
ENV PATH="/root/.cargo/bin:${PATH}"
<<<<<<< Updated upstream
RUN apt update
RUN apt install curl -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN apt-get update && apt-get install build-essential -y
RUN apt-get install binutils-dev
RUN apt-get install libunwind-dev -y 
RUN apt install git -y
RUN cargo install honggfuzz
RUN cargo install cargo-fuzz
RUN rustup toolchain install nightly
RUN  apt-get install pkg-config -y
RUN apt-get install binutils-dev

# New layer to clone the code
RUN mkdir /app
WORKDIR /app
RUN git clone https://github.com/lambdaclass/fuzzing_examples.git

# Second construction stage
FROM debian
WORKDIR /code

COPY --from=builder /app /code/fuzzing_examples
=======
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
>>>>>>> Stashed changes
