#!/bin/sh

podman build \
    -f Containerfile.flatpak-node \
    -t frontier-flatpak/node

podman run --rm \
    --network=host \
    -v $PWD:/usr/src/frontier:rw,Z \
    frontier-flatpak/node

podman build \
    -f Containerfile.flatpak-cargo \
    -t frontier-flatpak/cargo

podman run --rm \
    --network=host \
    -v $PWD:/usr/src/frontier:rw,Z \
    frontier-flatpak/cargo
