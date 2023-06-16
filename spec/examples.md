# Examples

## "Hello World!"

A simple hello world example. Uses platform imports and aliases.

```beach
// Narrows platforms by support, preventing compile time errors
needs stdout;
// Indicates that the platform should not error if it does not have keep_console_open
wants keep_console_open;

// Creates an alias: print refers to platform~stdout
print => platform~stdout;

// Runs on program start
main {
    // Print "Hello world!" to stdout
    print("Hello world!");
    // Keep the console open, if possible.
    keep_console_open;
};
```

## Variables

Variables are defined with the `let` keyword. Type casting is done using the becomes operator (`->`) for infalliable conversions. Types are implied to be the smallest datatype that can store a value.

```beach
needs stdout;
wants keep_console_open;

print => platform~stdout;

main {
    let x = 5;
    let y = 10;
    let z = x * y;

    print(z -> string);

    keep_console_open;
};
```

## Basic Functions

All functions are variables. "Standard" (const/immutable) functions can be declared in global/file scoping, like other constant variables. Functions are defined as any input variables and their types inside of the closure bars (`|`), followed by the becomes operator and the return type.

```beach
needs stdout;
wants keep_console_open;

print => platform~stdout;

main {
    let z = multiply(x, y);
    print(z -> string);
    keep_console_open;
};

let multiply = |a: integer, b: integer| -> integer {
    return a * b;
};
```

## Special Function Considerations

Note that functions have some special rules:

- All functions must have a return type. If no return value is needed, use `nothing`.
- All functions must have at least one argument. If no arguments are needed, use `nothing`. An example is provided:

```beach
let weird_function = |nothing| -> nothing { ... };
```

- All functions must return from every branch. If the function is a void function this is done with a simple `return` keyword.
  - As an exception to the previous rule, if a function *never* returns, it may have the `never` return type. These functions *may not* return, under *any* circumstance.
- Calling a function with no arguments may be done without including the parentheses after the funciton name.
- Functions can be declared in any scope. The exception is mutable functions, which must be declared locally.

## Scope

The following scopes exist in beach:

- Global scope
- Main scope
- Local scope
- Function scope

This order can be followed most to least global. (e.g. Global is in Main, Main is in Local, ect.)
Variables in Global scope *must* be constant.

## General Types

|type     |members                      |usage          |operators               |
|---------|-----------------------------|---------------|------------------------|
|integer  |u/i8;16;32;64;128;256, bigint|whole numbers  |+,-,/,*,**,==,!=,&,|,^,%|
|number   |f32;64, bigfloat, fraction   |real numbers   |+,-,/,*,**,=~,!~,%      |
|complex  |c64;128, bigcplx             |complex numbers|+,-,*,**,=~,!~          |<!-- more operators should be possible here, look into this -->
|printable|string, str, char            |text           |+,==,!=                 |
|...      |...                          |...            |...                     |
|nothing  |N/A                          |void fns       |                        |
|never    |N/A                          |divergent fns  |                        |
