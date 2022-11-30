# nwbview


[![Rust](https://github.com/brainhack-ch/nwbview/actions/workflows/rust.yml/badge.svg)](https://github.com/brainhack-ch/nwbview/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/nwbview.svg)](https://crates.io/crates/nwbview)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/brainhack-ch/nwbview/blob/master/LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`nwbview` is a software to display the contents of the binary NWB file format. It is written in Rust for high-performance, memory safety and ease of deployment.

## NWB format

[Neurodata Without Borders (NWB)](https://www.nwb.org/) is a standard data format to represent neurophysiology data files. It enables interoperability between neurophysiology data produced by different neuroscience labs. Examples of the data stored in .NWB format range from patch clamp experiments to optical physiology experiments.

The underlying storage technology used by the NWB format is the binary [HDF format](https://en.wikipedia.org/wiki/Hierarchical_Data_Format). While storing the files as binary brings advantages on the read/write speed as well as the file size, it stores the data in a way that is not readable by humans. I.e. one cannot open the file in a text editor to see the contents, thus there is need for a viewer.

## GUI

`nwbview` uses the [egui](https://github.com/emilk/egui)  Rust GUI library for rendering.

## To install and run using cargo

First install the `cargo` package manager and then run the following command to install `nwbview`.

```shell
cargo install nwbview
```

Once you completed the installation, simply type `nwbview` on the console to run it.

```shell
nwbview
```


## To build and run from the source code

The Rust library dependencies are provided in the `cargo.toml` file.

Note that the Rust libraries depend on the following system packages that need to be provided.

* `libgtk-3-dev`
* `librust-atk-dev`
* `libhdf5-serial-dev`

The exact names of the packages may differ between systems.

Once all the dependencies are satisfied, go to the directory containing `cargo.toml` and run the following command.

```shell
cargo run --release
```

The release flag builds the artifacts with optimizations. Do not specify it when you need to debug.



## How to contribute

All contributions are welcome and very much appreciated :)

We use Github's issue tracker, pull requests and the discussion interfaces for the contributions.

The pull requests require the approval from a maintainer as well as the CI checks.
