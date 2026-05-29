This module is the WIP system for creating local file structures for projects
---
---

## Architecture Overview

---
### hexagonal architecture with the core of the module features a struct with traits that:
+ creates the base dir for the programs local storages data files and the dir structures that needed
+ find the location for the base dir for the storage system or get the default location for that program per os system
    + the set location will most likely be done though arg inputs the program pass as an absolute path to that location. this also means that there needs to be another helper function ddownstream of the core that can resolave/get the it from the user
        + the downstream helper will also need a way to find the root as a starting point
+ create/edit and load files in the right place in the storage structure

+ hold a working tree struct of the storage system for fast lookup
    + the tree struct will be a helper struct thats referenced in the main fms struct
    + needs to have a helper that's implemented by the program that uses the fms to create the shape of the storage system that it expects


### the core struct also has a helper builder streuct with traits that:

### 


___

## External crates used:
+ sysinfo
+ tokio
+ os_info

## Internal used:
+ Path
+ Cstring
+ env