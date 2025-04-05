#!/usr/bin/env bash

set -e

cp -v ./target/release/run-pipeline ~/bin/run-pipeline-new

if [ -e ~/bin/run-pipeline ]
then
    rm -v ~/bin/run-pipeline
fi

mv -v ~/bin/run-pipeline-new ~/bin/run-pipeline
