# beach 🏖️

Beach is an open-source programming language built to make creating multi-platform code effortless.

## State of the Language

Beach is in a very early state and is not ready to be used in any development.

## Installing

TODO

## Examples

### Code

```beach
include std:stdio:stdout;

print => stdout;

main {
    print("Hello World!");
}
```

### Running locally

```terminal
> beach run
beach 🏖️ v0.0.0
👓 Parsing file...
🔨 Compiling for macos-aarch64...
📦 Packaging as mach-o...
▶️ Running...

Hello World!
```

### Cross compiling

```terminal
> beach build
beach 🏖️ v0.0.0
👓 Parsing file...
🎛️ Calculating valid targets...
🔨 Compiling for macos-aarch64...
📦 Packaging as mach-o...
📦 Packaging as *.app...
🔨 Compiling for macos-amd64...
📦 Packaging as mach-o...
📦 Packaging as *.app...
🔨 Compiling for windows-amd64...
📦 Packaging as pe32+...
...
☑️ Done!
```

Seriously, it's that easy. No installing alternate target packages, no required
host platforms, nothing. Just `beach build` and let beach figure out what
platforms your code could work on.
