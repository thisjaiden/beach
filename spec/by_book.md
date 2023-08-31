# Basic Specification

## aliasing

```beach
function_to_alias => aliased_name;
function_to_alias_externally =>! aliased_name;
```

Creates an alias in this file. Adding an ! to the end of the alias symbol exports this alias to other files importing this one.

## main

```beach
main {
    // Code here!
    return;
};
```

The first thing to run in your program. A type of function with no arguments in and no return value out. Special consideration: the main block may return `nothing` *or* `never`. No need to clarify which, but one must be returned.

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
let name = value;
// To specifiy type used.
let name: type = value;
// To create a mutable variable
let mut name = value;
```

## logical operations

```beach
let a = true;
let b = false;

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

## namespaces

```beach
/// main.beach
// Imports all global items of a under the `a~` prefix
import a;
// a -> b -> foo: follow the chain and add `~`s
a~b~foo(5, 10);

/// a.beach
// Imports all global items of b under the `b~` prefix, but also reexports these items
import b;

/// a/b.beach
// generic function
let foo = |a: integer, b: integer| -> nothing { ... }
```

## glob imports

```beach
/// main.beach
import a~~
// Valid since all items under foo were imported to this namespace
bar;

/// foo.beach
let bar = |nothing| -> nothing { ... }
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

## functions

```beach
// all functions are variables.
let add_two_numbers = |a: number, b:number| -> number {
    return a + b;
}

// functions that do not take inputs must be written so:
let the_number_five = |nothing| -> number {
    return 5;
}

// functions that do not return a value must be written so:
let i_do_nothing = |nothing| -> nothing {
    return;
}

/*
 * Other notes:
 * - All functions must return from all branches
 * - As functions are variables, they can only be mutable in non-global scope
 */
```

## conditionals

```beach
if a == b {
    // runs if condition is met
}
else if a < b {
    // runs if all previous conditions are not met, and this condition is
}
else {
    // runs if all previous conditions are not met
}
```
