# Resolve Clipboard

Your easy way to paste images directly into DaVinci Resolve Studio.  

> [!IMPORTANT]  
> This only works on **Windows** with the **Studio** version of DaVinci Resolve.  

## Installation

1. Head over to the [Releases](https://github.com/VilleOlof/resolve_clipboard/releases) page and download the latest version.  
2. Just run the executable when you want to paste an image into your mediapool.  

Easy as that!

## Other tidbits

The program will create a new folder in your mediapool called 'Clipboard' where it will store the images you paste.  
The actual images are stored in `%APPDATA%/resolve_clipboards/{project}/{timestamp}.png`  

## Development

Currently there is no way for other people to build and compile this project.  
This is due to that the dependency `resolve_api` is a work in progress project.  
Just make an issue if you got any bugs or feature requests.  