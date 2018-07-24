FROM rust:latest

RUN apt-get update && \
    apt-get upgrade -y --no-install-recommends && \
    apt-get install -y --no-install-recommends \
        curl \
        git \
        zsh

ARG APP_DIR
RUN mkdir $APP_DIR
WORKDIR $APP_DIR
COPY . .
