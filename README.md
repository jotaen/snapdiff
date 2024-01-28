# snapdiff

snapdiff compares two snapshots of a directory tree, captured at different points in time.
(Think of a “snapshot” as a backup of the original directory tree, in the sense of a full copy.)
That way, it gives a high-level insight into how the directory tree has evolved over time.

Learn more in [this blog post](https://www.jotaen.net/iE3XC).

### Example

Say, you want to compare two snapshots, one taken at 2023-09-01, and another one taken at 2023-10-01:

```
$ snapdiff 2023-09-01/ 2023-10-01/

                           FILES             BYTES
                                     G   M   K   B
TOTAL       Snap 1        87,243    98,407,188,994
            Snap 2        87,319    98,591,738,372
            
OF WHICH    Identical     87,134    97,551,550,976
            Moved             38       134,217,728
            Added             87       234,881,024
            Deleted           11        50,331,648
            Modified         147       671,088,644 (+282,172)
```

The categories are defined as:

- **Identical**: both snapshots contain a file at the same path with the same contents.
- **Moved**: both snapshots contain a file with the same contents, but at different paths.
- **Added**: the second snapshot contains a file whose path or contents is not present in the first snapshot.
- **Deleted**: the first snapshot contains a file whose path or contents is not present in the second snapshot.
- **Modified**: both snapshots contain a file at the same path, but with different contents.

Note: the files count doesn’t include folders.

## Usage

```
snapdiff
    [--report PATH]
    [--include-dot-paths]
    [--include-symlinks]
    [--workers N] OR [--workers N1:N2]
    [--no-color]
    SNAP1 SNAP2
```

Run `snapdiff --help` for all details.

## Build from Sources

Prerequisites: Rust toolchain (see [`Cargo.toml`](./Cargo.toml) for required version).

Compile via `cargo build --release`. (Produces binary to `target/release/snapdiff`.)

## About

snapdiff was created by [Jan Heuermann](https://www.jotaen.net). The sources are available under the terms of the [MIT license](./LICENSE).
