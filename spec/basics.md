# Basic Specification

## aliasing

```beach
function_to_alias => aliased_name
function_to_alias_externally =>! aliased_name
```

Creates an alias in this file. Adding an ! to the end of the alias symbol exports this alias to other files importing this one.

## main

```beach
main {
    // Code here!
};
```

The first thing to run in your program. A type of function with no arguments in and no return value out.

## comments

```beach
// a one line comment
/*
 A multi-line comment!
*/
```

## variables

```beach
// Implied type from context. Usually the smallest type that functions.
var name = value;
// To specifiy type used.
var name: type = value;
```

## logical operations

```beach
var a = true;
var b = false;

a == b  // False (IS)
a != b  // True  (IS NOT)
a && b  // False (AND)
a || b  // True  (OR)
a ^^ b  // True  (XOR)
!a == b // True  (NOT)

```

## external files

```beach
// Imports a local file (file_name.beach)
import file_name;
// Imports a library with a specified version
import library_name~1.2.3;
// Imports a library with a different top level name
import library_name~1.2.3 => namespace_alias;
```

## typecasting

```beach
// Typecasting between basic types
let a: u16 = 1234;
let b: i32 = 1234 -> i32;
```

## arithmetic

```beach
a + b //  Addition
a - b //  Subtraction
a / b //  Division
a * b //  Multiplication
a % b //  Modulo
a ** b // Power
a & b //  Bitwise AND
a | b //  Bitwise OR
a ^ b //  Bitwise XOR
```
