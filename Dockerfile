FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get -qq update
RUN apt-get -qq install -y wget git debhelper cmake pkg-config jq yq > /dev/null

CMD ["/shared/inside-docker.sh"]
