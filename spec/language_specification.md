# beach

Specifications for a compatible beach compiler. These specifications are not currently stable.

## A. Definitions

1. "Whitespace" is defined by the following characters:
    - Space
    - `\n`
    - `\r`
    - `\t`
2. "Main block" is defined at [D.1.*](#d-code-blocks) in this specification <!-- Proper location! And hyperlink :) -->
3. "Identifier" refers to any name for a variable, type, trait, or enum that meets the standards described under [C.1/C.2/C.3](#c-keywords).
4. "Opening brace" refers to `{`.
5. "Closing brace" refers to `}`.
6. "Project" refers to a complete program/library and its code. Imported libraries are seperate projects, local code files are not.
7. "Library projects" are projects who can be imported by other projects. They have additional limitations and features.
8. "Comments" are lines of text beginning with `//`. These are usually ignored.
    1. A multi-line comment may be created by opening with `/*` and closing with `*/`.

## B. Files

1. Code files must have a *.beach format. Any other extension must be treated as unusuable until renamed. By default, the root file in a project is `main.beach`.
2. The folder containing `main.beach` is considered the project folder. The name of this folder should match the project's name.
3. Placing config files for the compiler besides code files and not in them using the `system` keyword ([C.4](todo)) is highly discouraged.
4. Files included by the `file` keyword ([C.?](todo)) must use the file extension `.beach`.
5. Filepaths are platform-agnostic.
    1. Directories are seperated by a forward slash (`/`).
    2. The home directory is specified by starting a path with a tilda (`~`).
        1. The home directory is located at `/Users/USERNAME/...` on MacOS.
        2. The home directory is located at `C:\Users\USERNAME\...` on Windows.
        3. The home directory is located at `/home/USERNAME/...` on Linux.
    3. Going up one directory level is specified by two dots and a slash (`../`).
6. Running programs will look for filepaths not prefixed by a slash or tilda by starting next to the executable.
7. Programs run directly after compilation (e.g. `beach run`) look for files not prefixed by a slash or tilda starting out of the project folder.
8. Program output binaries are located at `./build/binaries/...` relative to the project folder.
9. Program output intermediates are located at `./build/intermediates/...` relative to the project folder.

## C. Keywords

1. All keywords *cannot* be used as names for variables, types, traits, or enums. They are reserved solely for their purpose.
2. Keywords must be prepended by one of the following characters/conditions:
    - Whitespace
    - `;`
    - Start of file
3. A list of keywords is as follows:
    - main ([D.1](#d-code-blocks))
    - type
    - trait
    - enum
    - public
    - namespace
    - library
    - include
    - file
    - let
    - for
    - return
    - in
    - system
4. The `system` keyword is reserved for use by the compiler. Anything between the end of the keyword and the next semicolon/line break is not required to be in any format or use any specification. This is the recommended way to add compiler flags/configuration.

## D. Code Blocks

1. The main block is a code block run apon a program's execution, if present.
    1. The main block is started with a `main` keyword followed by an opening brace. This block must be ended with a closing brace.
    2. There may be whitespace before the brace.
    3. The main block may not contain opening or closing braces except for in the following case:
    4. The main block may contain closing braces that do not count as the end of the block. There must be as many opening braces inside the main block as closing braces.
    5. There may only be one main block per project.
    6. The main block is not allowed in library projects.
    7. The main block is not required in any project.
    8. The main block may return either `never` or `nothing`. There is no determinisim on which is returned, so the `ending` type may be used to describe either `never` or `nothing`.
2. Code blocks are comprised of functional statements. [(E.*)](#e-functional-statements)
3. Functional statements in code blocks are seperated by a semicolon.
4. The last functional statement in a code block must be followed by a semicolon.
5. If a code block contains no functional statements, it must not have anything but whitespace and comments.

## E. Functional Statements

1. Functional statements are parts of a project file that consist of program logic and functions.
    1. "Logic" is a class of functional statement that may compute some result.
        1. Unused logic can be optimized out.
        2. Logic simple enough to compute during compilation may be optimized in this fashion.
        3. Logic is comprised of operators, types, and actions.
            1. Types convey some information or data. See [F.5](#f-types) for a full list of types.
            2. Operators convey an action to be taken with one or more values. A table of operators, their names, and their accepted values, and their completed operations is as follows:

                |operator|name|parameters|operation|
                |--------|----|----------|---------|
                | A + B | add | number | Adds A to B |
                | A - B | subtract| number | Subtracts B from A |
                | A * B | multiply | number | Multiplies A by B |
                | A / B | divide | number | Divides A by B |
                | A % B | modulo | number | Computes A modulo B |
                | A ** B | exponent | number | Raises A by B |
                | A & B | bitwise AND | number | ANDs the bits of A with the bits of B |
                | A \| B | bitwise OR | number | ORs the bits of A with the bits of B |
                | A ^ B | bitwise XOR | number | XORs the bits of A with the bits of B |
                | A && B | logical AND | boolean | Returns `true` if A and B are both `true`, or `false` otherwise |
                | A \|\| B | logical OR | boolean | Returns `true` if A or B are `true`, or `false` otherwise |
                | A ^^ B | logical XOR | boolean | Returns `true` if A or B are `true`, but `false` if they are both `true`. Returns `false` otherwise. |
                | A == B | equals | number, boolean, string | Returns `true` if A equals B, or `false` otherwise |
                | A != B | not equals | number, boolean, string | Returns `true` if A does not equal B, or `false` otherwise |
                | A > B | greater than | number | Returns `true` if A is greater than B, or `false` otherwise |
                | A < B | less than | number | Returns `true` if A is less than B, or `false` otherwise |
                | A >> B | bitshift right | integer | Shifts the bits of A right by B bits, filling with zeros. Overrides sign bits |
                | A << B | bitshift left | integer | Shifts the bits of A left by B bits, filling with zeros. Overrides sign bits |
                | A <<+ B | filling bitshift left | integer | Shifts the bits of A left by B bits, filling with ones. Overrides sign bits |

            3. Actions describe what to do with the completed result. A list of actions is as follows:
                1. Assignment is defined as a mutable identifier followed by `=` and logic that either conforms to the type of the identifier or can be converted to the type of the identifier losslessly.
                2. TODO: Calling
    

 - assignment
 - arithmetic
 - calls
 - generation


Scope & Functions

## F. Types

1. Types describe how a section of memory should be interpreted and allocated.
2. Types come in three forms: enumerations, structs, and compiler types.
    1. `enumeration` is often shortened to `enum`.
3. A fourth form of type, traits, can often be used in place of the other forms of types, but does not describe a specific type.
4. Describing a type as a "compiler" type is to say that it is implemented outside of the scope of what the other type types can achive.
5. A table of all built-in types and some basic information is provided.

    |type name  |example value(s)       |type type |notes             |
    |-----------|-----------------------|----------|------------------|
    | integer   | -6, 4, 12             | trait    |                  |
    | float     | -58.3, NaN, ∞         | trait    |                  |
    | number    | 58.3, -6, 12          | trait    |                  |
    | complex   | 23.5 + 14.0i          | trait    |                  |
    | string    | "Hello World!"        | struct   |                  |
    | boolean   | false                 | enum     |                  |
    | u8-u512   | 40                    | compiler | powers of two    |
    | i8-i512   | -6, 4, 12             | compiler | powers of two    |
    | f16-f128  | -58.3, NaN, ∞         | compiler | powers of two    |
    | c32-c256  | 23.5 + 14.0i          | compiler | powers of two    |
    | usize     | 0x12345678            | compiler | pointer width    |
    | isize     | -583815               | compiler | signed usize     |
    | ptr       | 0xDEADBEEF            | compiler | usize alias      |
    | function  | \|\| -> () {}         | struct   |                  |
    | error     |                       | trait    |                  |
    | printable | "Foo"                 | trait    |                  |
    | result    | Good(...), Error(...) | enum     |                  |
    | maybe     | Yes(...), No          | enum     |                  |
    | nothing   | ()                    | compiler | zero sized type  |
    | never     | !                     | compiler | not determinable |

6. Custom types can be created with the `enum`, `trait` and `type` keywords.
    1. Custom types cannot use any of the built-in type names. [F.5](#f-types)
    2. Custom types cannot use any keyword names. [C.1](#c-keywords)

 - Enums
 - Traits
 - Structs

Compiler Design

