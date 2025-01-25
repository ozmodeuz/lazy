#!/usr/bin/env bash

cd $(dirname $0)

nixpkgs=$(nix eval --raw -f ${PWD}/lazy.nix nixpkgs)
nix_path="nixpkgs=${nixpkgs}:nixos-config=${PWD}/lazy.nix"

sudo env NIX_PATH="${nix_path}" nixos-rebuild build-vm --fast "$@"