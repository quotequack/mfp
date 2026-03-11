# MultiFormatPicture

Image codecs should not wait for standardasation


## FAQ

### Why?

There are lots of image compression algorithms some even better than the more common ones that are not used cause of their lack of support. This project gives them a container to live inside and a program to let any image viewer open them 

### How?

When the image format is detected by using the codecID, its read and put into ram as a png (which is most probably the most supported format) and opened like that 
## Installation


## Install:

### Linux

```bash
git clone https://github.com/quotequack/mfp
cd mfp 
just install_all
```

### NixOS

This project contains an installable flake.nix, use that

### Windows/MacOS
*to be determined*


## Usage

### Creating a .mfp file

To contain an image of supported codec in an mfp file do:
```bash
mfpcreate INPUT_PATH CODEC OUTPUT_PATH
```
Input path: path of input image
Codec: codec to use (can be different than currect codec)
Output path: path for output image

### Opening a .mfp file

xdg-open should open the mfpread program
that means in your file browser of choice you should be able to double click a .mfp to open it with your default image viewe

For a cli option use:
```bash
mfpread INPUT_PATH
``` 
directly

### Extracting from a .mfp file

To extract the image inside a .mfp file do:
```bash
mfpextract INPUT_PATH
```
Image should be extracted into current directory as extract.XXX (XXX being the format it uses)

## Codecs

### Supported
* jpeg
* png
* bmp
* qoi


## Contributing

Contributing is easy!

See `contributing.md` for ways to get started.


## Roadmap

* more formats
* mfp converter