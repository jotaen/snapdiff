# snapdiff

Compare two snapshots of the same directory tree, which were taken at different times.

## Example invocation

```
$ snapdiff /mnt/backups/2023-05-28 /mnt/backups/2023-08-15
                         FILES         BYTES

TOTAL       Before      28.372   137.295.441
            After       28.405   138.481.078

OF WHICH    Unchanged   26.511   122.670.012
            Moved            3        19.523
            Added           23       +26.288
            Deleted          1        -4.677
            Modified     1.039        +3.512  (+12.004 / -8.492)
```
