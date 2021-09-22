# 🐌 esc

Small CLI for escaping characters in strings.

## Install

```bash
cargo install esc
```

## Usage

```bash
cat LICENSE-MIT | esc escape | pbcopy
```

```bash
pbpaste | esc unescape | pbcopy
```

**Note** This reads the entire input in memory at once in order to locate paired characters.


## Acknowledgments

This is just a tiny wrapper over [snailquote](https://docs.rs/snailquote/0.3.0/snailquote/) which does all the actual work here.