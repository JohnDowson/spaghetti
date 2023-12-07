FROM centos:7

RUN yum -y update && \
    yum -y install \
    ca-certificates \
    curl \
    gcc \
    glibc-devel

# Install rust
RUN mkdir /rust
WORKDIR /rust
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --profile minimal --default-toolchain stable -y
WORKDIR /
ENV PATH=/root/.cargo/bin:$PATH

# Cache dependencies
RUN USER=root cargo new --bin spaghetti
WORKDIR /spaghetti
COPY ./maud ./maud
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release

# Build application
COPY ./migrations ./migrations
ENV DATABASE_URL=postgres://spaghetti@localhost/spaghetti-dev
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/spaghetti*
RUN cargo build --verbose --release
