# beach 🏖️

Beach is an open-source programming language built to make creating multi-platform code effortless.

## State of the Language

Beach is in a very early state and is not ready to be used in any development.

## Installing

TODO

## Examples

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

Seriously, it's that easy. No installing alternate target packages, no required host platforms,
nothing. Just `beach build` and let beach figure out what platforms your code could work on.

### Interpreter

Yes, it comes with an interpreter too!

```terminal
> beach live
beach 🏖️ v0.0.0
READY
B> let a = 4;
B> needs stdout;
B> print(a -> string);
4
B> 
```

It's not meant for production, and it can't do everything, but it's there!
