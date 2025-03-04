# terms.md

```txt

                ┌──────────────────────┐                                              
                │                      │                                              
      ┌Converted► Abstract Syntax Tree ├───Converted──────┐                           
      │         │                      │                  │                           
      │         └──────────────────────┘                  │                           
┌─────┴────────┐        ┌────Prefixed────┐                │                           
│              │        │                │                │                           
│ Project File │        │                │                │                           
│              │        │                │                │                           
└─────▲────────┘ ┌──────▼─────┐  ┌───────┴────────┐       │                           
      │          │            │  │                │       │                           
      │          │ Input File │  │ std/core.beach │       │                           
      │          │            │  │                │       │                           
  Dependencies   └─────┬──────┘  └────────────────┘       │                           
   Resolved            │                              Appended                        
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
