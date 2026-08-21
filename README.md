# Resolve Clipboard

The easiest way to paste images directly into DaVinci Resolve Studio.

> [!IMPORTANT]  
> This only works on **Windows** with the **Studio** version of DaVinci Resolve.  

## Installation

1. Head over to the [Releases](https://github.com/VilleOlof/resolve_clipboard/releases) page and download the latest version.  
2. Just run the executable when you want to paste an image into your mediapool.  

Easy as that!

## Paths for Images

The program will create a new folder in your mediapool called 'Clipboard' where it will store the images you paste.  
The actual images are stored in `%APPDATA%/resolve_clipboards/{project}/{timestamp}.png`  

This however can be overriden by setting `RESOLVE_CLIPBOARD_DIR` in your environmental variables.  
Then the path would be `{RESOLVE_CLIPBOARD_DIR}/{project}/{timestamp}.png`.  

## Building

ensure you have [`rustc`](https://rust-lang.org/) and it's `cargo` toolchain.  

```bash
cargo build --release
```