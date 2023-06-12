# beach 🏖️

Beach is an open-source programming language built to make creating multi-platform code effortless.

## State of the Language

Beach is in a very early state and is not ready to be used in production code.

## Installing

TODO

## Example

### Code

```beach
needs platform::stdout;

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
☑️ Built for 37 targets in 2:38.47
```
