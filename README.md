# prepend

Edit a file to insert lines on head.

See [help.txt](./src/help.txt).

## Installation

You need cargo (Rust build tool).

```sh
cargo install --git https://github.com/vain0x/prepend
```

## Motivation

Since pipe-like works concurrently, this doesn't work as you expected:

```sh
# a.txt has some data.
echo world >a.txt

# Prepend to a.txt. (NOT WORK)
echo hello | cat a.txt - >a.txt

# See what happened.
cat a.txt
#=> hello
```

Because `>a.txt` truncates all data from `a.txt` first. `cat` opens it and reads nothing.

## With prepend command

```sh
echo world >a.txt
prepend a.txt hello

cat a.txt
#=> hello
#   world
```

## Examples

See [tests/data](tests/data).
