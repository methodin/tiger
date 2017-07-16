# musl build image
FROM ekidd/rust-musl-builder:stable

MAINTAINER Derek Woods <derekrw@gmail.com> (@methodin)

ADD ./ /home/rust/src

RUN sudo chmod -R 777 /home/rust/src/target

ENV SSL_CERT_DIR=/etc/ssl/certs

WORKDIR /home/rust/src

RUN cargo build --release

WORKDIR /tiger

ENTRYPOINT ["/home/rust/src/target/x86_64-unknown-linux-musl/release/tiger"]
