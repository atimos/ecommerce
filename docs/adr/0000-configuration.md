# Configuration

* Status: accepted
* Date: 2021-01-13

## Context and Problem Statement

The merchant needs to be able to configure the system by region or by store.
There should be a default settings so they dont need to manually handle all stores/regions
It should be easy to understand what config is used when


## Considered Options

* cascading - system -> region -> store
* cascading - system -> store -> region
* configurable cascading
* configurable source per settings

## Decision Outcome

Chosen option: "configurable source per setting", because it allowes the most flexible configuration.
the missing flow of overrides can be managed by a UX showing all ovverides
UX can help with reasoning about what is overridden, either by having a page showing all overrides or
show it on every setting somehow

### Positive Consequences

* Allows for more merchant constellations

### Negative Consequences

* No nice flow to reason about config

## Pros and Cons of the Options

### cascading - system -> region -> store

Add store to region, store overrides region, region overrides system

* Good, easy to understand, store always overrides region and region always overrides system
* Good, easy to implement
* Good, easy to visualize
* Good, does support multiple stores in one region
* Good, does support one region with multiple stores
* Bad, does not support one store with multiple regions

### cascading - system -> store -> region

Add region to store, region overrides store, store overrides system

* Good, easy to understand, region always overrides store and store always overrides system
* Good, easy to implement
* Good, easy to visualize
* Good, does support one store with multiple regions
* Bad, does not support one region with multiple stores

### configurable cascading

Add region to store, region overrides store, store overrides system
But can on store level switch it to store overrides region, region overrides system

* Good, does support one store with multiple regions
* Good, does support one region with multiple stores
* Good, easy to implement
* Meh, not super easy to visualize
* Meh, not super easy to understand

### configurable source per settings

Add region to store,
every setting has a source [system, store, region] that
defaults to system and can be changed on store level.
A lists of all overrides could help to make it easier to find, handle and reset the overrides.

* Good, does support one store with multiple regions
* Good, does support one region with multiple stores
* Good, easy to implement
* Good, easy to understand
* Good, easy to visualize
* Bad, hard to get a good overview and reason about overrides

<!-- markdownlint-disable-file MD013 -->
