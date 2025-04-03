FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"

COPY --chmod=777 docker-bootstrap.sh /bin/docker-bootstrap.sh
COPY --chmod=777 target/release/build-deb-package /bin/build-deb-package

RUN /bin/docker-bootstrap.sh

ENTRYPOINT ["/bin/build-deb-package", "run"]
