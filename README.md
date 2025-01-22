# pomobar-rs

`pomobar-rs` is a Waybar plugin with a Pomodoro timer.
The plugin uses `socket` communication between Waybar and pomobar.

---

## Install

Build with `Cargo`:

```shell
cargo build
```

The binaries will be in `target/release/pomobar` (server) and `target/release/pomobar-cli` (client).

---

## Usage

And start the server:

```shell
pomobar
```

Use `pomobar-cli` to show current status:

Show status and count down the timer

```shell
pomobar-cli status
```

Toggle timer

```shell
pomobar-cli toggle
```

Reset timer

```shell
pomobar-cli reset
```

---

Inspired by: [mt190502/pomobar](https://github.com/mt190502/pomobar)
