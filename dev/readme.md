# /dev

This folder contains a variety of scripts and tools to assist in the development
of beach itself. Nothing in here is useful to the end-user.

## update_std.sh

This script copies the current std files present in the parent folder to the
installation location for beach. This allows quick iteration and testing of
changes to code in the standard library, as all beach builds look for the
installation folder to gather std, including debug/local builds.
