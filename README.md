# cega

## CGA/EGA (the graphics formats from 80s games) parsing with png/gif/etc output and colored terminal preview

[![github](https://img.shields.io/badge/github-knzai/cega-8da0cb?logo=github)](https://github.com/knzai/cega)
[![crates.io](https://img.shields.io/crates/v/cega?logo=crates.io)](https://crates.io/crates/cega)
[![docs.rs](https://img.shields.io/docsrs/cega?logo=docs.rs)](https://docs.rs/cega/latest/cega/)
[![Crates.io](https://img.shields.io/crates/d/cega?style=flat-square)](https://crates.io/crates/cega)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/knzai/cega/commit.yml)](https://github.com/knzai/cega/actions/workflows/commit.yml)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

## Warning

This is currently very much in an alpha state: apis and CLI arguments may still change heavily and the apis aren't documented. This file has decent usage examples below.

## Installation dependencies

If you use feature `sdl2` you must install the sdl2 libraries first

## MacOS
`brew install sdl2 sdl2_gfx sdl2_image`

## Linux
`[sudo] apt-get install libsdl2-dev libsdl2-gfx-dev libsdl2-image-dev`

## Roadmap/Upcoming

### Priority
- [x] ~~gif or png output (half the point of the whole library).~~
- [x] ~~If I use image::DynamicImage there are additional encoders/decoders that will be free~~
- [ ] Zooming/scaling (for png and gui output)
- [ ] Additional EGA planar encodings
- [ ] Map viewing (the other half): using the tile/spritesheets as palettes for larger images in psuedo CGA/EGA form (common in 80s games)
- [ ] Outputting to CGA/EGA encodings - useful for making non copyrighted test format files to ship

### Everything else
- [ ] Document the code beyond a simple readme
- [ ] Palettes beyond the defaults- cga from the full 16 and ega from the full 256.
- [x] ~~Breaking the features back out now that I have a better understanding of the modules~~
- [ ] More tests. Adding as I go, but as I learn rust more and understand the problem space better will make sense to do more
- [ ] Full paletting from RGB(?A alpha support maybe)
- [ ] Better terminal support - rascii for iterm/sixel/etc or ratitui for full tui (helpful for discovering formats) and scrollbars
- [ ] Optimize terminal output (don't repeat active codes)

### Great if I get to it, but a bit of a tangent
- [ ] Train a model to recognize format and tiling patterns for smart discovery. I don't know how many test files I can get my hands on but I guess I could generate them

## Binary/Terminal Usage

This can be used as a library or executable.

The binary handles arguments (and thus help) via [clap 4](https://crates.io/crates/clap) 

```console
Usage: cega [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  

Options:
  -a, --ascii-mode <ASCII_MODE>      images will horizontally crop to terminal
                                     [possible values: a, c, p, h]
                                     a = plain ascii
                                     c = colored ascii
                                     p = full pixels via ansi bg color
                                     h = horizontal half pixels (UGLY)
  -p, --palette [<PALETTE>]          ega palette can be used for cga, but not the inverse
                                      [possible values: cga0, cga0i, cga1, cga1i, ega]
  -i, --image-parser <IMAGE_PARSER>  [default: cga] [possible values: ega_row_planar, erp, cga]
  -c, --custom-ascii <CUSTOM_ASCII>  4 or 16 chars palette like -a " +%0"
  -w, --width <WIDTH>                [default: 320]
  -m, --max-width <MAX_WIDTH>        used for wrapping rows if retiling with tile_height
  -o, --output-file <OUTPUT_FILE>    format based on extension - see image crate
  -t, --tile-height <TILE_HEIGHT>    
  -s, --sdl                          
  -q, --quiet                        
  -h, --help                         Print help
  -V, --version                      Print version
```
cega can parse output to png, and [other formats](https://docs.rs/image/latest/image/codecs/index.html#supported-formats).

cega can parse tiled/spritesheet style cga and output "pixels" to the terminal

```cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t p```:

<img width="650" alt="cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t p" src="https://github.com/knzconnor/cega/assets/53/5aa8478e-dcdc-4637-8bc4-539059e1a5d6">

cega will output some suggestions, like if it's not a CGA fullscreen image, but you don't specify tiling:

```cega ../../assets/game/CGATILES.BIN -t a```:

<img width="650" alt="cega ../../assets/game/CGATILES.BIN -t a" src="https://github.com/knzconnor/cega/assets/53/1aa38c4d-1b3c-44eb-b716-67e0768734c3">

cega will ouput in different preview formats, such as colored ASCII or a gui window:

```cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t c -c "1234" -s```:

<img width="650" alt="cega ../../assets/game/CGATILES.BIN -w 16 -r 16 -i cga -t c -c 1234 -s" src="https://github.com/knzconnor/cega/assets/53/593e9c9f-2780-4201-af93-7073155e876c">

## Cargo features and library usage

```toml
default = ["terminal", "sdl2", "png"]
terminal = ["clap"]
png = ["image"]
```

Disabling terminal will skip the binary target
Png feature is handled via [image](https://crates.io/crates/image) so includes that dep

## Acknowledgements & References
 - [The DOS Game Modding Wiki](https://moddingwiki.shikadi.net/wiki/Main_Page), particularly [User:TheAlmightyGuru](https://moddingwiki.shikadi.net/wiki/User:TheAlmightyGuru)
   - [Raw_EGA_data](https://moddingwiki.shikadi.net/wiki/Raw_EGA_data)
   - [Ultima_I_Tile_Graphic_Format](https://moddingwiki.shikadi.net/wiki/Ultima_I_Tile_Graphic_Format)