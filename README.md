# zloopctl - Control zloop devices

## Why zloopctl
While the zloop driver has a very simple interface that can even be used by
shell scripts I found that some users might have different needs like e.g.:

- listing the available zloop devices
- a more intuitive way of creating the add command
- etc...

## Usage:
```
Control zloop devices

Usage: zloopctl [OPTIONS] [COMMAND]

Commands:
  list  list zloop devices
  add   add zloop device
  del   delete zloop device
  help  Print this message or the help of the given subcommand(s)

Options:
  -d, --debug    Enable debug output
  -h, --help     Print help
  -V, --version  Print version
```

## Why Rust
Because I felt like it.
