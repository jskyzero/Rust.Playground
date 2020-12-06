#!/bin/bash
for filename in src/proto/*.proto; do
  # build proto (make sure you hava add protoc to your path)
  protoc --proto_path=src/proto \
        --rust_out=src/proto.rs \
        $filename
done