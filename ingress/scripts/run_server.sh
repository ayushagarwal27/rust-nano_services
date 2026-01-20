#!/usr/bin/env bash
# File: ingress/scripts/run_server.sh
# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..
cd frontend
npm install
npm run build
cd ..
cargo clean
cargo run