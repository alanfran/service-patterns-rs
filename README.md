# Rust Service Patterns
This is a companion repo to [service-patterns-go](https://github.com/alanfran/service-patterns-go) that shows how Rust implements the same functionality.

## Enum
Rust has first-class enums with compile-time gurantees to their correctnes.

This repo shows two ways to define an enum type that can be converted to/from a String:

1. `src/model/device.rs` - Manually implements type conversion traits from the standard library.
2. `src/model/device_strum.rs` - Uses a derive macro from the `strum` crate to generate the conversion traits. This module has only 13 lines of code. Compare that to the pseudo-enum in the Go repo that is capable of representing invalid states, and takes significantly more code.