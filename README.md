# check-crates

```
check-crates 0.1.0

USAGE:
	check-crates [FLAGS] [OPTIONS] [--] [ARGS]

FLAGS:
    -c, --compile       create a Cargo.toml containing the relevant crates and `cargo build` it instead of printing to stdout
    -h, --help          Prints help information
    -s, --sole-owner    only include crates for which the given user is the sole owner
    -V, --version       Prints version information

OPTIONS:
    -i, --ignore <IGNORE>...    ignore this crate

ARGS:
    id    the user id to find crates for
```
