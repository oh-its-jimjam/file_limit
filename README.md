# file_limit

A utility library to retrieve and set the file limits for the underlying process of Unix-based systems. 

"Everything is a file"; thus a file can represent network sockets, pipes, files on a file-system, etc.
Resource-intensive applications typically need to increase the default limit to the maximum. It is also beneficial for low-latency applications to increase the file limit to the maximum number and pre-allocate a data structure to hold the FDs to minimize memory allocations. 

## Get the current file limit

```rust
let cur_limit: usize = file_limit::get();
```

## Get the number of maximum files that can be opened

```rust
let max_limit = file_limit::max();
match max_limit {
  Limit::Val(v) => ..., //v is the upper bound represented as a usize
  Limit::Inf => ..., //infinity repesents that there is no upper bound
}
```

## Set the file limit to the maximum upper-bound

```rust
let new_lim = file_limit::set_to_max().unwrap();
```

If there is no upper-bound (set to infinity) then it will scale it by factor of 8.

## To-do list

1. Add support for Windows since it uses `HANDLE` as the equivalent of a unix file.

I welcome PRs to add support for Windows.
