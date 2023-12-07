#!/bin/sh
set -ex
docker build --network=host -t spaghetti . &&\
docker run --name spaghetti1 spaghetti &&\
docker cp spaghetti1:/spaghetti/target/release/spaghetti ./spaghetti.bin &&\
docker rm spaghetti1
strip -g spaghetti.bin
