# Programming language

* Status: accepted
* Date: 20-11-21

## Context and Problem Statement

What language should be used for main development

## Decision Drivers

* Should be fun
* Should require static typing
* Should have support for wasm
* Should be conservative with computer resources
* Should work both on serverside and clientside

## Considered Options

* Rust

## Decision Outcome

Chosen option: "Rust", it ticks all the boxes

### Positive Consequences

* Code could be shared between frontend and backend
* Type system can help with specification

### Negative Consequences

* Slow compilation times
* Will make the development slower
