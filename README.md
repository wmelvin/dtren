# dtren

`dtren` renames a file by appending its last-modified timestamp to the file name in the form `YYYYmmdd_HHMMSS`.

Example: `test.txt` modified at `2025-11-10 09:08:07` becomes `test-20251110_090807.txt`.

## Usage

```
dtren <file>
```

Run `dtren -h` or `dtren --help` to display usage information.

## Behavior

- You can specify files using relative paths or absolute paths
- Tilde expansion (`~/`) is supported for home directory references
- The file's last-modified timestamp is used to generate the new name
- If a file with the new name already exists, `dtren` will exit with an error to avoid overwriting files
- Symbolic links are not supported - `dtren` will exit with an error if you try to rename a symlink
- Directories cannot be renamed with `dtren`
