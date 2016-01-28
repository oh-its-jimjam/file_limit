# file_limit

A utility library to retrieve and set the file limits for unix-based processes. 

"Everything is a file"; thus a file can represent network sockets, pipes, file on a file-system, etc.
Resource-intensive applications typically need to increase the default limit to the maximum. It is also beneficial for low-latency applications to increase the file limit to the maximum number and pre-allocate a data structure to hold the FDs to minimize memory allocations. 

## Get the current file limit

```
let cur_limit: usize = filelimit::get();
```

## Get the number of maximum files that can be opened

```
let max_limit = filelimit::max();
match max_limit {
  Limit::Val(v) => ..., //v is the upper bound represented as a usize
  Limit::Inf => ..., //infinity repesents that there is no upper bound
}
```

## Set the file limit to the maximum upper-bound

```
let new_lim = filelimit::set_to_max().unwrap();
```

If there is no upper-bound (set to infinity) then it will scale it by factor of 8.

## To-do list

1. Add support for Windows since it uses `HANDLE` as the equivalent of a unix file.

I welcome PRs to add support for Windows.
