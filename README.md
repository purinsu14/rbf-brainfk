# rbf - Brainfuck Interpreter in Rust

A fast and lightweight Brainfuck interpreter written in Rust.

## Features

* Fast and efficient Rust implementation
* Simple command-line interface
* Debug mode for inspecting execution state
* Step-by-step execution for easier debugging
* Error handling for unmatched brackets

## Installation

Clone the repository and install:

```bash
git clone https://github.com/purinsu14/rbf-brainfk.git
cd rbf-brainfk
make install
```

The `rbf` executable will be installed and available from anywhere in your system.

## Usage

Run a Brainfuck program:

```bash
rbf <file.bf>
```

Example:

```bash
rbf hello.bf
```

### Debugging

Print memory and instruction state while executing:

```bash
rbf --debug <file.bf>
```

Execute one instruction at a time:

```bash
rbf --debug --step <file.bf>
```

## Flags

| Flag      | Description                                                             |
| --------- | ----------------------------------------------------------------------- |
| `--debug` | Print memory and instruction state at each step                         |
| `--step`  | Step through the program one instruction at a time (requires `--debug`) |

## Uninstall

```bash
make uninstall
make clean
```

## Contributing

Contributions are welcome. If you find a bug or have an idea for an improvement, please open an issue or submit a pull request.

Issues:
https://github.com/purinsu14/rbf-brainfk/issues

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Made by [purinsu14](https://github.com/purinsu14)
