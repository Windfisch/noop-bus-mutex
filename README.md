noop-bus-mutex
==============

This is a [BusMutex](https://docs.rs/shared-bus/0.1.4/shared_bus/mutex/trait.BusMutex.html)
implementation for the [shared-bus](https://github.com/Rahix/shared-bus) crate in rust that
does nothing.

*shared-bus* is a crate to allow sharing bus peripherals safely between multiple devices.

**noop-bus-mutex** is appropriate whenever all devices that share a bus are only accessible
by *a single* execution context (i.e., all devices are accessed only by the main() thread
or only by some interrupt handler). In this situation, no locking is required, which is
exactly what this crate does.
