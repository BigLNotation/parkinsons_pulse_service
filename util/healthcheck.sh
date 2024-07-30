#!/bin/bash

if [[ -z "${METRICS_PORT}" ]]; then
  PORT=2222 # Need to make sure this is equal to DEFAULT_APT_PORT in ./src/config.rs
else
  # shellcheck disable=SC2034
  PORT=$METRICS_PORT
fi

curl -f http://localhost:"$PORT"/health
