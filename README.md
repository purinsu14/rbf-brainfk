## rbf — Brainfuck Interpreter in Rust
A brainfuck interpreter written in Rust.

## Install
```
git clone https://github.com/purinsu14/rbf-brainfk.git
cd rbf-brainfk
make install
```
Then use it from anywhere:
```
rbf <file.bf>
```

Example:
```
rbf hello.bf
```

## Usage
```
rbf <file.bf>
rbf --debug <file.bf> 
rbf --debug --step <file.bf>
```

## Flags
| Flag | Description |
|------|-------------|
| `--debug` | Print memory and instruction state at each step |
| `--step` | Step through the program one instruction at a time (only works with `--debug`) |

## Uninstall
```
make uninstall
make clean
```

---
*Made by [purinsu14](https://github.com/purinsu14)*
