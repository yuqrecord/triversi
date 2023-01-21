# Triversi

Triversi is a Reversi-like game played by 3 players.

![Game image](images/game_image.png)

## Installation

Compilation requires the `cargo` command, so if you do not have it,
refer to [this page](https://www.rust-lang.org/ja/tools/install) and install it.

In order to install Triversi, execute the following commands
(refer to [this site](https://doc.rust-lang.org/cargo/commands/cargo-install.html)).

```sh
cargo install --git https://github.com/yu1guana/triversi
```

If you want to use the alternate key bindings, excute the following commands.

```sh
cargo install --git https://github.com/yu1guana/triversi --features alternative_key_binding
```

## Usage

```text
Usage: triversi [OPTIONS]

Options:
  -r, --range <RANGE>                Number of positions in one edge (>= 5 & = 0,2 (mod3)) [default: 14]
  -d, --distance <DISTANCE>          Distance between positions (>= 2, <= 10) [default: 3]
  -p, --player-names <PLAYER_NAMES>  Marks of each player (delimiters are ','),  [default: Cyan,Magenta,Yellow]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Key Bindings

Key bindings are displayed at the top when playing.

Two key bindings (default and alternative) are supported.
If you want to change key bindings, edit the source code ([src/app/key\_binding.rs](src/app/key_binding.rs)).

The difference of key bindings between default and alternative is as follows:

|           | default | alternative |
|:-         | :-      | :-          |
|Move up    | k       | i           |
|Move down  | j       | k           |
|Move left  | h       | j           |
|Move right | l       | l           |
|History    | t       | h           |

## To-Do list

See [.todos.toml](.todos.toml).
Using [git-todos](https://github.com/yu1guana/git-todos), you can read a To-Do list confortably.

## License
Copyright (c) 2023 Yuichi Ishida  
Released under the MIT license  
[https://opensource.org/licenses/mit-license.php](https://opensource.org/licenses/mit-license.php)
