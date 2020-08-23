FROM ubuntu:latest

MAINTAINER Dmitry Abakumov <killerinshadow2@gmail.com>

RUN apt-get update && apt-get -y install rust postgresql redis node diesel-cli

RUN mkdir /app

COPY * /app

RUN cd /app &&  move configurations/config.example.toml > configurations/config.toml

