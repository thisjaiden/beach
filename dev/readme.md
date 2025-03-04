# /dev

This folder contains a variety of scripts and tools to assist in the development
of beach itself. Nothing in here is useful to the end-user.

## /spec/language_specification.md

Full language specification. If you make a compiler that meets this spec, it's a
fully compatible/compliant beach compiler.

## terms.md

General project layout and terms information.

## update_std.sh

This script copies the current std files present in the parent folder to the
installation location for beach. This allows quick iteration and testing of
changes to code in the standard library, as all beach builds look for the
installation folder to gather std, including debug/local builds.
