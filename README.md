# README #



### A place to keep my Rust stuff ###

* Learning the Rust language

### Installation of Rust ###

* Download package from [rustup](https://www.rust-lang.org/en-US/other-installers.html#more-rustup)
* Unpack by `tar -zvxf rust-*-.tar.gz` 
* In the expanded directory `./install.sh --prefix=your/installation/dir/`

### Start a new package ###
Create an executable

```
cargo new <project name> --new
```
Create a library

```
cargo new <project name>
```
### Test ###
Run test by

```
cargo test
```
Cargo suppress the standard output by default.  Enable standard output by

```
cargo test -- --nocapture
```
  