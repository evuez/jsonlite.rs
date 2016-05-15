#!/usr/bin/env bash
JSONLITE_EXEC="./target/release/jsonlite"

ITERATIONS=500

for i in $(seq 1 $ITERATIONS); do
  ID=$($JSONLITE_EXEC set '{"name":"John Doe","active":true,"permissions":{"read":true,"write":false}}')
  $JSONLITE_EXEC get "$ID" > /dev/null
  $JSONLITE_EXEC delete "$ID"
done
