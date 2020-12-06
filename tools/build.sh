#!/bin/bash
for filename in config/proto/*.proto; do
  # build proto (make sure you hava add protoc to your path)
  protoc --proto_path=config/proto \
        --rust_out=config/src/ \
        $filename
done