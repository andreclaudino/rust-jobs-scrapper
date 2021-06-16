FROM ubuntu:20.10

# gcr.io/ml-team-307117/indeed-scrapper

ADD indeed-scrapper /usr/bin/indeed-scrapper

RUN apt-get update && \
    apt-get -y install libssl1.1 libssl-dev ca-certificates && \
    apt-get -y clean autoclean && \
    apt-get -y autoremove --yes && \
    rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["indeed-scrapper"]