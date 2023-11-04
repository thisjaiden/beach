# Basic Specification

## aliasing

```beach
function_to_alias => aliased_name;
function_to_alias_externally =>! aliased_name;
```

Creates an alias (alternate name) for a function.  Adding an ! to the end of the alias symbol exports this alias to other files importing this one.

## main

```beach
main {
    // Code here!
    return;
};
```

The first thing to run in your program. A type of function with no arguments in and no return value out. Special consideration: the main block may return `nothing` *or* `never`. No need to clarify which, but one in specific must be returned.

Special notes:

- Projects that are executables *must* have a main block.
- Projects that are library code and not executables may not have a main block.
- Projects that are bare metal or operating system agnostic *must* have a main block, but it may not be imediately executed.

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
// To create a mutable variable--one that can be changed.
var mut name = value;
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
// Imports a local file. (file_name.beach)
file file_name;
// Imports a library with a specified version. The version is required.
library library_name~1.2.3;
// Imports a library with a different top level name.
library library_name~1.2.3 => namespace_alias;
```

## namespaces

> main.beach

```beach
// Imports all exported items of `a` under the `a~` prefix.
file a;

// a -> b -> foo: follow the chain and add `~`s
a~b~foo(5, 10);
```

> a.beach

```beach
// Imports all exported items of b under the `b~` prefix, but also reexports these items
file b;
export b;
```

> a/b.beach

```beach
// generic function
var foo = |a: integer, b: integer| -> nothing { ... };
// exports said function
export foo;
```

## glob imports

> main.beach

```beach
file foo~~
// Valid since all items under foo were imported to this namespace.
bar;
// This also works with libraries.
```

> foo.beach

```beach
var bar = |nothing| -> nothing { ... };
export bar;
```

## `result`s

Sometimes functions return a result\<T>. This indicates that either the function errored, or succeeded and is returning a value of type T. This is a builtin type.

## typecasting

```beach
// Typecasting between basic types
var a: u16 = 1234;
var b: i32 = 1234 -> i32;

// Catching errors when typecasting
var big: u32 = 100000;
// .typecast() returns a result<u8>, and running .expect on that either converts
// to a u8 if the conversion was successful, or crashes with a message if there
// was an error converting.
var smaller: u8 = big.typecast().expect("unable to typecast!");
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
var add_two_numbers = |a: number, b:number| -> number {
    return a + b;
}

// functions that do not take inputs must be written so:
var the_number_five = |nothing| -> number {
    return 5;
}

// functions that do not return a value must be written so:
var i_do_nothing = |nothing| -> nothing {
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
