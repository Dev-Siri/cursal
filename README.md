# Cursal

A half-baked, functional curl like HTTP client for the command line, written in Rust.

This is only my second rust project so.. uhh.. you might notice some... strange snippets.

## Getting Started

- Clone the repository

```sh
$ git clone https://github.com/Dev-Siri/cursal
```

- Make sure you have Rust installed and then run the build command:

```sh
$ cargo build --release
```

- If you want to install it as a global binary, then use cargo's install commadn instead:
```sh
$ cargo install --path=.

# now you can use it anywhere
$ cursal https://google.com
```

## License

This project is MIT licensed, see [LICENSE](LICNESE)