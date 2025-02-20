# nubuild

Abstracts away build systems by executing some command based on some definition. (linux only at the moment)

## Configuration

The configuration is stored in a file named `nubuild.yml` which is stored in `~/.config/nubuild/nubuild.yml`, and looks like this:

```yml
- name: cargo
  file: Cargo.toml
  commands:
    build: [cargo, build]
    run: [cargo, run]
  default: build
  separator: --
- name: make
  file: Makefile
  commands:
    build: [make, -f, Makefile]
  default: build
```

This can be overriden locally by creating a file named `nubuild.yml` in the current directly.

### name

The name that can be used to call that specific command if there are several matches, because the first match will be called by default.

### file

The file that needs to be detected to match that specific command.

### commands

Subcommands that can be called by naming them directly.

### default

The name of one of the `commands` that is called if none are called directly.

### separator (optional)

If the command need a separator to accept additional arguments.

## Examples

Situation: we are inside a Rust crate folder, and our configuration file is the one at the start.

```console
$ nubuild
$ nubuild cargo
$ nubuild build
$ nubuild cargo build
```

All those commands are identical and will run `cargo build`.

The separator is to avoid having to type it yourself each time.

```console
$ nubuild run test
$ cargo run -- test # will expands to this
```

But you can type it yourself if you need to pass arguments to the command (or just don't set that field).

```console
$ nubuild run --bin runner -- file.txt
$ cargo run --bin runner -- file.txt # will expands to this
```
