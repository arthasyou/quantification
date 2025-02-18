#!/bin/bash

docker rm -f surrealdb
docker run -d \
    --name surrealdb \
    --restart=always \
    -v `pwd`/authdata:/authdata \
    -p 8000:8000 \
    surrealdb/surrealdb:latest start --user root --pass root rocksdb:/authdata/authbase.db