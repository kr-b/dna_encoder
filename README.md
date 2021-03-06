# DNA Encoder [![forthebadge](https://forthebadge.com/images/badges/built-with-science.svg)](https://forthebadge.com)
## Rust program to convert data into a DNA sequence

*DISCLAIMER:* This isn't legit or anything, just a small project I wanted to try out..
It's simply a base 4 converter that masks the bits with a set key which is the labels of the 4 different bases.

## Usage
```
cargo run <encode/decode> <plain text type> <key>
```
Then input your data through the app or pipe it in

### Input Types
- integer/int
- string/str

### Key
Any 4 character non-repeating permutation of the four DNA bases (A, C, G and T)

#### Example "ACGT"
Basically means this:

| 0 | 1 | 2 | 3 |
|---|---|---|---|
| A | C | G | T |

Therefore, the number 37 with key ACGT would be represented like this:

| 4<sup>3</sup> | 4<sup>2</sup> | 4<sup>1</sup> | 4<sup>0</sup> |
|---|---|---|---|
| 0 | 2 | 1 | 1 |
| A | G | C | C |

## Examples
This converts a string into 4 bit nibbles of DNA bases
```
$ cargo run enc str CGAT
[<]  Input: Hello!
[>] Output: GCAC GAGG GATC GATC GATT CACG
```
This converts an integer into a long DNA sequence
```
$ cargo run e int ACGT
[<]  Input: 2048
[>] Output: GAAAAA
$ cargo run encode integer ACGT
[<]  Input: 100
[>] Output: CGCA

```

## TODO
- [x] Encoding
- [x] Decoding
- [ ] Decoding: Automatic key detection
- [ ] Put this data on actual DNA (never gonna happen but I want this on the list)
