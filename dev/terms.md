# terms.md

In the installation directory (%HOME%/beach), there are three main folders and one file. `core` and `core.beach` describe and include all fundamental language code that is automatically compiled as a part of any project. This is mostly fundamental types and traits. `intrinsics` describes anything outside of the scope of what can be written into the standard library itself: pointer widths, syscall numbers, ect. `std` is the standard library of functions, types, and traits which can be included with the `include` keyword.

```txt

                ┌──────────────────────┐                                              
                │                      │                                              
      ┌Converted► Abstract Syntax Tree ├───Converted──────┐                           
      │         │                      │                  │                           
      │         └──────────────────────┘                  │                           
┌─────┴────────┐ |      ┌────Prefixed────┐                │                           
│              │ |      │                │                │                           
│ Project File │ |      │                │                │                           
│              │-|      │                │                │                           
└─────▲────────┘|┌──────▼─────┐  ┌───────┴────────┐       │                           
      │         |│            │  │                │       │                           
      │         |│ Input File │  │ std/core.beach │       │                           
      │         |│            │  │                │       │                           
  Dependencies  |└─────┬──────┘  └────────────────┘       │                           
   Resolved <---|      │                              Appended                        
      │            Converted                              │                           
      │                │                                  │                           
      │          ┌─────▼────────────────┐              ┌──▼──────────────────────────┐
      │          │                      │              │                             │
      └──────────┤ Abstract Syntax Tree ├───Converted──► Intermediate Representation │
                 │                      │              │                             │
                 └──────────────────────┘              └─┬───────────────────────────┘
                                                         │                            
                 ┌────────────────────┐                  │                            
                 │                    │                  │                            
                 │ Platform Assembly  ◄─────Converted────┘                            
                 │                    │                                               
                 └────────────────────┘                                               
```
