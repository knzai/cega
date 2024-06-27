# cega

> **CGA/EGA graphics binary file parsing for Rust**

[![Crates.io](https://img.shields.io/crates/v/cega?style=flat-square)](https://crates.io/crates/cega)
[![Crates.io](https://img.shields.io/crates/d/cega?style=flat-square)](https://crates.io/crates/cega)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Contributors](https://img.shields.io/github/contributors/knzconnor/cega?style=flat-square)](https://github.com/knzconnor/cega/graphs/contributors)

## Warning

This is currently very much in an alpha state. EGA is not currently implemented. Output to a usable file format is not either. It's currently a preview tool that uses ANSI escape codes for a terminal view and SDL2 for a gui preview window.

## Usage

This can be used as a library or executable. Pending finalizing internal API I'll demonstrate the CLI usage instead (check main.rs for how to call etc).

The binary handles arguments (and thus help) via [clap 4](https://crates.io/crates/clap) 

```console
     Running `target/debug/cega --help`
Usage: cega [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  

Options:
  -t, --terminal-output <TERMINAL_OUTPUT>
          [possible values: a, c, p, h]
          a = plain ascii
          c = colored ascii
          p = full pixels via ansi bg color
          h = horizontal half pixels
          Images may be wider than terminal and will then crop
  -p, --palette [<PALETTE>]
          [default: 1] [possible values: 0, 0i, 1, 1i]
  -c, --custom-ascii <CUSTOM_ASCII>
          4 chars palette like -a " +%0"
  -w, --width <WIDTH>
          
  -m, --max-width <MAX_WIDTH>
          [default: 320]
  -r, --retile-height <RETILE_HEIGHT>
          
  -s, --sdl
          
  -q, --quiet
          
  -i, --image-type <IMAGE_TYPE>
          [default: cga] [possible values: cga, ega]
  -h, --help
          Print help
  -V, --version
          Print version

```


cega can parse tiled/spritesheet style cga and output "pixels" to the terminal

```cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t p```:

<img width="650" alt="cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t p" src="https://github.com/knzconnor/cega/assets/53/5aa8478e-dcdc-4637-8bc4-539059e1a5d6">

cega will output some suggestions, like if it's not a CGA fullscreen image, but you don't specify tiling:

```cega ../../assets/game/CGATILES.BIN -t a```:

<img width="650" alt="cega ../../assets/game/CGATILES.BIN -t a" src="https://github.com/knzconnor/cega/assets/53/1aa38c4d-1b3c-44eb-b716-67e0768734c3">

cega will ouput in different preview formats, such as colored ASCII or a gui window:

```cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t c -c "1234" -s```:

<img width="650" alt="cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t c -c 1234 -s" src="https://github.com/knzconnor/cega/assets/53/593e9c9f-2780-4201-af93-7073155e876c">

