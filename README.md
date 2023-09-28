Certainly! Below is a `README.md` file you can use for your `backlight` Rust project. Make sure to fill in the sections marked with `TODO` as appropriate for your project:

```markdown
# Backlight Control

Backlight Control is a simple command-line utility written in Rust for controlling the backlight brightness on Linux laptops.

## Features

- Displays current backlight brightness as a percentage with a corresponding icon.
- Allows increasing or decreasing the backlight brightness.

## Usage

```bash
backlight             # Displays the current backlight brightness
backlight up          # Increases the backlight brightness
backlight down        # Decreases the backlight brightness
```

## Example

```bash
$ backlight
 11%
$ backlight up
 16%
$ backlight down
 11%
... and so on
```

## Installation

```bash
cargo install --path .
```

### Prerequisites

- Rust compiler installed on a Linux system.
- Proper permissions to read and write to `/sys/class/backlight/<device>/brightness` and `/sys/class/backlight/<device>/max_brightness`.
- supported (untested) /sys/class/backlight/amdgpu_bl0/
- suppored (tested)  /sys/class/backlight/intel_backlight/

## Development

To contribute to Backlight Control, follow these steps:

1. Fork the repository.
2. Create a new branch with your feature or bug fix.
3. Commit your changes, and push them to your fork.
4. Submit a pull request to merge your changes to the main repository.

