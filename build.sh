#!/bin/bash

cargo build --release

# Check if dist directory exists, if not, create it
if [ ! -d "dist" ]; then
    mkdir dist
fi

cp -r config dist/
cp .env dist/
cp target/release/quantification_rs dist/
 
