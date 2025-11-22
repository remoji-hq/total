# `total`

Total station helper tool.

## Installation

```
cargo install total
```

## Usage

```
total convert -f sdr -t dxf samples/nikon-test-sdr33-coord.txt
```

Tested with Nikon NPL-332.

## Supported formats

Input: 

* [x] Sokkia SDR2x/SDR33 Coordinates
* [ ] Sokkia SDR2x/SDR33 Raw
* [x] Nikon Coordinates
* [ ] Nikon Raw
* [x] AutoCAD DXF
* [ ] GeoJSON

Output: 

* [x] Nikon Coordinates
* [x] AutoCAD DXF
* [ ] GeoJSON

# License

The source code is licensed under the [MIT license](LICENSE).
