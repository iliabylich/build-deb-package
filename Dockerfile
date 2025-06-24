FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"

COPY --chmod=777 container-bootstrap.sh /bin/container-bootstrap.sh
COPY --chmod=777 target/release/build-deb-package /bin/build-deb-package

RUN /bin/container-bootstrap.sh

ENTRYPOINT ["/bin/build-deb-package", "run"]
