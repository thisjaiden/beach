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
