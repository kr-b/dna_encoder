# DNA Encoder
## Rust program to convert data into a DNA sequence

*DISCLAIMER:* This isn't legit or anything, just a small project I wanted to try out..
It's simply a base 4 converter that masks the bits with a set key which is the labels of the 4 different bases.

## Usage
```
cargo run <input type> <key>
```
Then input your data through the app or pipe it in

### Input Types
- integer/int
- string/str

### Key
Any 4 character non-repeating permutation of the four DNA bases (A, C, G and T)

#### Example "ACGT"
Basically means this:

>A  C  G  T
>3<sup>3</sup> 3<sup>2</sup> 3<sup>1</sup> 3<sup>0</sup>
>[] [] [] []

## Examples
This converts a string into 4 bit nibbles of DNA bases
```
cargo run str CGAT
```
