# dtren

`dtren` renames a file by appending its last-modified timestamp to the file-stem in the form `YYYYmmdd_HHMMSS`.

Example: `test.txt` modified at `2025-11-10 09:08:07` becomes `test-20251110_090807.txt`.

Usage
```
dtren <file>
```

Behavior
- If no argument is provided: exits non-zero and prints usage.
- If `-h` or `--help` is provided: prints usage and exits with code 1.
- Accepts relative paths and `~/` (tilde) expansion and resolves to an absolute path.
- Errors when the path does not exist, or the path is a directory or a symlink (symlinks are disallowed by spec).
- If the computed destination file already exists, the program exits with an error.

Build (Linux)

On Linux, from the project root:

```bash
cargo build --release
# Release binary: target/release/dtren
```

Build (Windows)

To produce a Windows executable (`dtren.exe`) from Linux you can cross-compile. A typical target is `x86_64-pc-windows-gnu`.

You will need to install the target and a suitable linker (for example, mingw-w64). Example steps:

```bash
rustup target add x86_64-pc-windows-gnu
# install mingw-w64 (platform-specific; on Debian/Ubuntu: apt install mingw-w64)
cargo build --target x86_64-pc-windows-gnu --release
# Windows exe: target/x86_64-pc-windows-gnu/release/dtren.exe
```

If you prefer a smoother cross-build experience, consider using `cross` (https://github.com/rust-embedded/cross).

Notes
- The program intentionally treats symlinks as errors per specification. If you want symlinks to be followed, modify `symlink_metadata` usage accordingly.
