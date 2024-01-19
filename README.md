# snapdiff

snapdiff compares two snapshots of a directory tree, which were taken at different times, and summarises the difference between them. I.e., how many files have moved, modified, added, or deleted.

```
$ snapdiff my-backups/2023-05-01 my-backups/2023-06-01
                         FILES         BYTES

TOTAL       Before      28.372   137.295.441
            After       28.405   138.481.078

OF WHICH    Unchanged   26.511   122.670.012
            Moved            3        19.523
            Added           23        26.288
            Deleted          1         4.677
            Modified     1.039        12.004 (+3.512)
```

## Usage

### `--report-file`, `-r`

Example: `--report-file ./my-report.txt`

Print a detailed report to a file.

### `--include-dot-paths`

Include files and folders whose name start with a dot (`.`), instead of ignoring them (which is the default).

### `--include-symlinks`

Resolve symlinks, instead of ignoring them (which is the default).

### `--workers`

Example: `--workers 4` or `--workers 1:8`

The number of workers (CPU kernels) to utilize.

`0` means that it detects the number of available cores automatically (which is the default).

You can use two different values, separated by a colon (`:`), to differentiate for the first and the second snapshot.

### `--no-color`

Print output in plain text, without colouring.

## Build from Sources

Prerequisites: Rust toolchain (see [`Cargo.toml`](./Cargo.toml) for required version).

Build with `cargo build`.

## About

snapdiff was created by [Jan Heuermann](https://www.jotaen.net), and is available under the terms of the [MIT license](./LICENSE).
