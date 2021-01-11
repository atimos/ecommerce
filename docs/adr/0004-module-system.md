# Module system

* Status: draft
* Date: 2021-01-16

## Context and Problem Statement

To be able to allow third parties to integrate to the system some kind of module system needs to be in place

## Considered Options

* one wasm file per module containing freestanding functions
* one wasm file per subsystem function

## Decision Outcome

Chosen option: "[option 1]", because [justification. e.g., only option, which meets k.o. criterion decision driver | which resolves force force | … | comes out best (see below)].

### Positive Consequences <!-- optional -->

* …

### Negative Consequences <!-- optional -->

* …

## Pros and Cons of the Options

### wasm one file per module containing freestanding functions

a module is one wasm file exporting functions and every function call is a new instance
checking function signatures against the systems requirements during module installation

* Meh, the public functions will need to be prefixes
* Meh, a module touching multiple subsystem will have a massive amount of functions exported in that file
* Bad, all host functions needs to be imported, makes it really hard to verify if a function is using the correct subsystem calls at installation step
  the validation will need to be handled by runtime checks, one bad solution is to return error if a subsystem function call is made in wrong context

### one wasmfile per subsystem function

a module is a zipfile containing one file per function that is expoesd in the system

* Good, the function imports can be checked at installation time
* Good, access control and host function import can be made for every function
* Meh, build step for module authors could be a bit messy because of the split into multiple files
* Bad, easy to miss adding a new host function for every module function call,
  can be mitigated by grouping the hosted funtions and import one or multiple groups of functions
* Bad, shared code in the module will need to be duplicated in every function file and consume more resources

<!-- markdownlint-disable-file MD013 -->
