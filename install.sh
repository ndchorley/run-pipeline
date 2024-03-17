#!/usr/bin/env bash

set -e

cp -v ./target/release/run-pipeline ~/bin/run-pipeline-new &&
    rm -v ~/bin/run-pipeline &&
    mv -v ~/bin/run-pipeline-new ~/bin/run-pipeline
