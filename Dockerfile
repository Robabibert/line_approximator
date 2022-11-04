FROM rust as base

WORKDIR /line_approximator

RUN apt update && apt install -y  gcc \
                        libc-dev\
                        git\
                        zsh\
                        htop\
                        curl\
                        pkg-config

RUN rustup default nightly
RUN rustup component add rls --toolchain nightly-x86_64-unknown-linux-gnu 
RUN rustup component add rust-analysis --toolchain nightly-x86_64-unknown-linux-gnu 
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup component add rustfmt
RUN cargo install cargo-docs

FROM base as src

COPY ./src src
COPY ./Cargo.toml Cargo.toml



###########START NEW IMAGE : ENVIRONMENT  ###################
FROM src as environment



###########START NEW IMAGE : DEVELOPMENT  ###################
FROM environment as development
ENV DOCKER_TARGET="development"
CMD exec /bin/bash -c "trap : TERM INT; sleep infinity & wait"

