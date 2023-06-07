## Hello World!
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

## General Types
|type     |members                      |usage          |operators              |
|---------|-----------------------------|---------------|-----------------------|
|integer  |u/i8;16;32;64;128;256, bigint|whole numbers  |+,-,/,*,==,!=,&&,||,^,%|
|number   |f32;64, bigfloat, fraction   |real numbers   |+,-,/,*,=~,!~,^,%      |
|complex  |c64;128, bigcplx             |complex numbers|+,-,*,=~,!~,^          |<!-- more operators should be possible here, look into this -->
|printable|string, str, char            |text           |+,==,!=                |
|...      |...                          |...            |...                    |
