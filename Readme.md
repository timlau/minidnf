# An example binary to dnf5daemon lib located here

[rust_dnfdaemon](https://github.com/timlau/rust_dnf5daemon)

## How to run
Checkout the rust_dnf5daemon repository in the same base directory as minidnf.

```
<basedir>/minidnf
<basedir>/rust_dnfdaemon
```

The module contains a simple binary there do the same as, but using the dnf5daemon server.

* `dnf list <pattern>`
* `dnf install <pattern>`
* `dnf remove  <pattern>`

### Examples (using cargo run)
```bash

cargo run -- install 0xFFFF
cargo run -- remove 0xFFFF
cargo run -- list dnf5* yum* --scope installed
```

### Usage
```bash

Usage: minidnf [OPTIONS] [COMMAND]

Commands:
  install  Install packages
  remove   Remove packages
  list     
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --debug    packages to search for Enable debug logging
  -h, --help     Print help
  -V, --version  Print version
```
