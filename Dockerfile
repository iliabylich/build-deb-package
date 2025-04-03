FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"

RUN apt update && \
    apt install --no-install-recommends -y \
        wget \
        curl \
        git \
        debhelper \
        cmake \
        pkg-config \
        jq \
        yq \
        g++ \
    && apt install -y ca-certificates && \
    update-ca-certificates && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup-init.sh && \
    chmod +x /tmp/rustup-init.sh && \
    /tmp/rustup-init.sh --profile minimal -y && \
    apt clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /tmp/*

COPY --chmod=777 target/release/build-deb-package /bin/build-deb-package

ENTRYPOINT ["/bin/build-deb-package", "run"]
