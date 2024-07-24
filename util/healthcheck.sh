#!/bin/bash

if [[ -z "${API_PORT}" ]]; then
  PORT=4444 # Need to make sure this is equal to DEFAULT_APT_PORT in ./src/config.rs
else
  # shellcheck disable=SC2034
  PORT=$API_PORT
fi

curl -f http://localhost:$PORT/health
