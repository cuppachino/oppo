# Dubble's Rust Workshop 🛠️

## Run & Reload `mod-<N>.rs`

This command clears the scrollbuffer and starts a watch server in the project directory.
We're using `-x` because `run` is a cargo commmand. If you need to run a shell command, use `-s` instead.

```ps1
clear && cargo watch -q -x 'run --bin mod-<N>'
```

## Run once

```ps1
cargo run --bin mod-<N>
```

## Modules

| Brief | Link |
| - | - |
| minimal type-state builder | [src/bin/mod-1.rs](src/bin/mod-1.rs) |
| isomorhpic type-state | [src/bin/mod-2.rs](src/bin/mod-2.rs) |
| expression-parser | [src/bin/mod-3.rs](src/bin/mod-3.rs) |
