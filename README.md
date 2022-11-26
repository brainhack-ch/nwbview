# nwbview

`nwbview` is a software to display the contents of the binary NWB file format. It is written in Rust for high-performance, memory safety and ease of deployment.

## NWB format

Neurodata Without Borders (NWB) [1] is a standard data format to represent neurophysiology data files. It enables interoperability between neurophysiology data produced by different neuroscience labs. Examples of the data stored in .NWB format range from patch clamp experiments to optical physiology experiments.

The underlying storage technology used by the NWB format is the binary HDF [2] format. While storing the files as binary brings advantages on the read/write speed as well as the file size, it stores the data in a way that is not readable by humans. I.e. one cannot open the file in a text editor to see the contents, thus there is need for a viewer.

## GUI

`nwbview` uses the `egui` [3] Rust GUI library for rendering.


## How to build

The Rust library dependencies are provided in the `cargo.toml` file.

Note that the Rust libraries depend on the following system packages that need to be provided.

* `libgtk-3-dev`
* `librust-atk-dev`
* `libhdf5-serial-dev`

Once all the dependencies are satisfied, go to the directory containing `cargo.toml` and run the following command.

```shell
cargo build
```

## How to run

Once again go to the directory containing the `cargo.toml` file and simply run the following command.

```shell
cargo run --release
```

The release flag builds the artifacts with optimizations. Do not specify it when you need to debug.



## How to contribute

All contributions are welcome and very much appreciated :)

We use Github's issue tracker, pull requests and the discussion interfaces for the contributions.

The pull requests require the approval from a maintainer as well as the CI checks.



## References

1. https://www.nwb.org/
2. https://en.wikipedia.org/wiki/Hierarchical_Data_Format
3. https://github.com/emilk/egui
