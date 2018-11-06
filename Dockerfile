FROM rust

WORKDIR /usr/src/test
COPY . .

RUN apt update && \
    apt remove -y 'libcurl*' && \
    apt install -y \
        build-essential \
        cmake \
        golang-go

RUN cargo install --path .

CMD ["chttp-issue-15"]
