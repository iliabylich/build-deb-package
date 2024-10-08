FROM ghcr.io/iliabylich/debian-unstable-builder:latest

ENV DEBIAN_FRONTEND=noninteractive

CMD ["/shared/inside-docker.sh"]
