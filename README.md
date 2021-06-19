# arrow_ring_buffer

A ring buffer designed to integrate well with [Apache Arrow](https://github.com/apache/arrow-rs).

## Design principles

 * Must allow smooth integraton with Apache Arrow.
 * Must perform exceptionally well, both natively or running on the JVM.
 * Must provide support for streaming applications.
 * Must provide support for window functions in the application code.
 * Borrows state of the art ideas and principles from both Rust and Scala ecosystems.

## Supported programming languages

* [Rust](rust/README.md)
* [Scala](scala/README.md)

## Sponsors

[Mathminds](http://mathminds.io) is the main contributor to `arrow_ring_buffer`.
