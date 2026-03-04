# rusty-matrix

`rusty-matrix` is a Matrix-like terminal screensaver written in Rust.
Inspired by [cmatrix](https://github.com/abishekvashok/cmatrix).

## Features

- rain effect
- Configurable colors
- Adjustable speed
- Interactive controls (change color and speed at runtime)

### Options

| Option        | Description                                                       | Default |
|---------------|-------------------------------------------------------------------|---------|
| `-c, --color` | Set matrix color (green, red, blue, white, yellow, cyan, magenta) | green   |
| `-d, --delay` | Set frame delay (e.g., 50ms, 100ms, 1s)                           | 50ms    |

### Keyboard Controls

|  Key  | Action        |
|:-----:|---------------|
|  `q`  | Quit          |
| `0-9` | Adjust speed  |
|  `r`  | Red color     |
|  `g`  | Green color   |
|  `b`  | Blue color    |
|  `w`  | White color   |
|  `y`  | Yellow color  |
|  `m`  | Magenta color |
|  `c`  | Cyan color    |