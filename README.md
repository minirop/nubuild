# nubuild

Abstract away build systems by executing some command based on some definition.

## Configuration

The configuration is stored in a file named `nubuild.yml` which is stored in `~/.config/nubuild/nubuild.yml` (unknown location for windows and MacOSX) and looks like this:

```yml
- name: cargo
  file: Cargo.toml
  commands:
    build: [cargo, --color=always, build]
    run: [cargo, --color=always,  run]
  separator: --
  default: build
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

### separator

If the command need a separator to accept additional arguments.

### default

The name of one of the `commands` that is called if none are called directly.

## Examples

Situation: we are inside a Rust crate folder, and our configuration file is the one at the start.

```console
$ nubuild
$ nubuild cargo
$ nubuild build
$ nubuild cargo build
```

All those commands are identical and will run `cargo --color=always build`.

The separator is to avoid having to type it yourself each time.

```console
$ nubuild run test
$ cargo run -- test # will expands to this
```
