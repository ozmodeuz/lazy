#!/usr/bin/env bash

cd $(dirname $0)

nixpkgs=$(nix eval --raw -f ${PWD}/nixos_config.nix nixpkgs)
nix_path="nixpkgs=${nixpkgs}:nixos-config=${PWD}/nixos-config.nix"

sudo env NIX_PATH="${nix_path}" nixos-rebuild build-vm --fast "$@"