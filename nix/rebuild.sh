#!/usr/bin/env bash

cd $(dirname $0)

nixpkgs=$(nix eval --raw -f ${PWD}/sysconf.nix nixpkgs)
nix_path="nixpkgs=${nixpkgs}:nixos-config=${PWD}/sysconf.nix"

sudo env NIX_PATH="${nix_path}" nixos-rebuild build-vm --fast "$@"