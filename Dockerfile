# first construction stage
FROM debian AS builder
WORKDIR /code
ENV PATH="/root/.cargo/bin:${PATH}"
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
