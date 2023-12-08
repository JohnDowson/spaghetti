#!/bin/sh
set -ex
cargo clean &&\
docker build --network=host -t spaghetti . &&\
docker run --name spaghetti1 spaghetti &&\
docker cp spaghetti1:/spaghetti/target/release/spaghetti ./spaghetti.bin &&\
docker rm spaghetti1 &&\
strip -g spaghetti.bin &&\
ssh -p 451 johnd@hjvt.dev 'sudo systemctl stop spaghetti.service' &&\
scp -P 451 ./spaghetti.bin spaghetti@hjvt.dev:/var/www/spaghetti/spaghetti.bin &&\
ssh -p 451 johnd@hjvt.dev 'sudo systemctl start spaghetti.service' &&\
rm ./spaghetti.bin
sass assets/:static/css/
scp -P 451 -r ./static/css spaghetti@hjvt.dev:/var/www/spaghetti/static/
