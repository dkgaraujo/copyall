# copyall

This terminal utility copies all or some files in a directory into a single file.

# Usage

Copy the content of all files in a directory into "output.out":

```
$ copyall
```

Copy the content of all files with the ".py" extension in a directory into "output.out":

```
$ copyall --ext py
```

Copy the content of all files with the ".py" extension and which start in "BKP" in a directory into "output.out":

```
$ copyall --ext py --regex "^BKP"
```

Copy the content of all files with the ".py" extension and which start in "BKP" in a directory into "output.json":

```
$ copyall --ext py --regex "^BKP" --json
```
