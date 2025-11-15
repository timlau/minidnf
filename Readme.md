# An example binary to dnf5daemon lib located here

[rust_dnfdaemon](https://github.com/timlau/rust_dnf5daemon)

## How to run
Checkout the rust_dnf5daemon repository in the same base directory as minidnf.

```
<basedir>/minidnf
<basedir>/rust_dnfdaemon
```

The module contains a simple binary there give an output like `dnf list <pattern>`

### Examples (using cargo run)
```bash
cargo run -- dnf5*
cargo run -- dnf5* yum* --scope installed
```

### Usage
```bash

Usage: minidnf [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  packages to search for

Options:
  --scope <SCOPE>      Package scope [default: all] [possible values: all, installed, available]
  -d, --debug          Enable debug logging
  -h, --help           Print help
  -V, --version        Print version
```
