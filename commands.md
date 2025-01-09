# Commands

## init

Create a default sysconf config and initialize it as a git repo.

1. check for existing config
2. if found warn user and require 'force' flag or something
3. if not found, create folder and copy in the 'init' versions of sysconf.nix, sysconf.toml, and .gitignore, then initialise the folder as a git repo

## add

Add packages and options.
add [-p] name [-t]
-p, --package
-t, --tag

## remove

Remove packages and options.

## search

Search for packages and options.
-p, --package
-o, --option
-i, --inputs

## update

Update all options and packages.

## clean

Remove old config generations.
-a, --age

## info

Print information about the system.

## input

Add an input.

## help

Print this message, or the help of the given subcommands.

## version

Print the version of sysenv.

## common arguments

--verbose
--quiet
--max-jobs
--impure
--cache
--refresh-cache
--offline

# Modules

## config

- pub enum ConfigFiles
  -- ::new
  -- .write_all()
  -- .write(name)

## errors

- pub enum Error
  -- Io
  -- Git
