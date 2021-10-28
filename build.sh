#!/bin/sh
docker build --add-host=host.docker.internal:host-gateway -t spaghetti . &&\
docker run --name spaghetti1 spaghetti &&\
docker cp spaghetti1:/spaghetti/target/release/spaghetti ./spaghetti.bin &&\
docker rm spaghetti1
strip spaghetti.bin