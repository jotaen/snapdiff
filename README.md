# snapdiff

snapdiff can compare two snapshots of a directory tree, which have been captured at different points in time.
(Think of a “snapshot” as a backup of the original directory tree, in the sense of a full copy.)
It diffs the two snapshots, and summarizes how many files are identical, and how many have been moved, modified, added, or deleted.
That way, you get a high-level insight into how the data evolved between both snapshots.

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

`SNAP1` and `SNAP2` are in “chronological” order, so snapshot 1 is assumed to precede snapshot 2.

See also `snapdiff --help` for info.

### `--report PATH`

Example: `--report ./my-report.txt`

Print a detailed report to a file.

The file will be newly created, so it fails if a file already exists at the target path. 

### `--include-dot-paths`

Include files and folders whose name start with a dot (`.`), instead of ignoring them (which is the default).

### `--include-symlinks`

Resolve symlinks, instead of ignoring them (which is the default).

### `--workers N`

Example: `--workers 4` or `--workers 1:8`

The number of workers (CPU cores) to utilize.

`0` means that it detects the number of available CPU cores automatically (which is the default).

You can specify two different values, separated by a colon (`:`), to differentiate between the first and the second snapshot.

### `--no-color`

Print output in plain text, without colouring.

## Build from Sources

Prerequisites: Rust toolchain (see [`Cargo.toml`](./Cargo.toml) for required version).

Compile via `cargo build`. (Produces binary to `target/debug/snapdiff`.)

## About

snapdiff was created by [Jan Heuermann](https://www.jotaen.net). The sources are available under the terms of the [MIT license](./LICENSE).
