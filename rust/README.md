# arrow_ring_buffer

A ring buffer designed to integrate well with [Apache Arrow](https://github.com/apache/arrow-rs).

## Use cases

### Missing data in CSV files

You would like to read a CSV from a file or from a stream. The data has missing fields, which causes Apache Arrow to abort the data ingestion.

A no-brainer solution would be downloading the CSV file or preprocessing it so that you remove those lines having missing fields. However, it would be far more preferable to process the data on the fly, removing those lines having missing fields as soon as they appear, passing to Apache Arrow only lines which are valid.

### Technical indicators for financial market data

You would like to obtain a stream of financial market data and implement a technical advisor, employing technical indicators such as RSI and alike. For example, you would like to add a column for RSI lagged by 9 periods. For more information about RSI: https://www.investopedia.com/terms/r/rsi.asp

A simple solution would be downloading the CSV file and perform a window functin which generates a new CSV file with the RSI column added. However, it would be far more preferable to add the RSI column on the fly, possibly other technical indicators as you see fit, passing to Apache Arrow all columns which are necessary for building an expert advisor.

## Requirements

 * Must allow smooth integraton with Apache Arrow.
 * Must provide support for window functions in the application code.
 * Must be lightwight and `no-std` compatible, for the benefit of embedded applications.
 * Must be zero runtime dependencies.
 * Must perform exceptionally well. Should employ branchless programming as much as possible.
 * Must provide a zero copy low level API for maximum performance.
 * Must provide a streams oriented API with support for back pressure.
 * Must provide a high level API which aggregates functionality from the low level API and streams API.

## Design Decisions

* O(1) cost in regime operation;
* Never allocates objects in regime operation;
* Not thread-safe, lock-free implementation: leave this concern to the caller;

### Compatibility with mixed language environments
In a mixed language environment, it is [recommended by Apache Arrow documentation](https://github.com/apache/arrow/blob/master/docs/source/format/Columnar.rst) that indexes *should* be limited to `2³¹-1`, since some programming languages do not offer unsigned integer arithmetic.

The feature `compat`, *which is selected by default*, makes sure that `i32` is employed in this case, as shown below:

```rust
#[cfg(feature = "compat")]
type Index = i32;

#[cfg(not(feature = "compat"))]
type Index = u64;
```

> Notice that formal function parameters are affected by the data type being employed. This is intentional: you'be better have compilation errors than silently employing potentially wrong or even hazardous indexes at runtime. More on this subject is explained in the next topic.

### Indexes are never tested for validity

The low level API *never* tests validity of indexes, due to performance reasons. It's responsibility of the calling code to make sure that indexes make sense.

> Indexes out of range may lead to undefined behavior. Negative indexes when `compat` feature is employed are hazardous and lead to security exploitation, since negative indexes give malicious code access to memory addresses outside the range reserved for the ring buffer.

## Status

This library is in early stage of development and it is likely that interfaces may change.

 * [x] low level API.
 * [ ] streams API.
 * [ ] high level API.

## Sponsors

[Mathminds](http://mathminds.io) is the main contributor to `arrow_ring_buffer`.
