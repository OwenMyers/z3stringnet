# Installation

Installing the compiler and the package manager (`cargo`)
can be done with a script online. Run:

```
curl https://sh.rustup.rs -sSf | sh
``` 

Choose option `1` when prompted

You may need to manually source the cargo env file to add it to your path

```
source $HOME/.cargo/env
```

# Compile

Move into the repo root directory and run:

```
cargo build
```

This compiles for debugging. If you want to compile and optimize:

```
cargo build --release
```

# Running

From the root:

```
target/debug/z3stringnet --size <int> --weights <float> --nbins 200000 --nmeasure 500 --nupdate 5
```

or if you compiled with the `--release` flag

```
target/release/z3stringnet <flags>
```

The options are both specified and found in human readable form in `src/cli.yml`

an example of a run would be

```
target/debug/z3stringnet --size 4 --weights 0.3
```

or if you compiled with the `--release` flag

```
target/release/z3stringnet <flags>
```

The options are both specified and found in human readable form in `src/cli.yml`

You will notice the the number of bins can't be set with a flag. I can fix this shortly.
right now you will have to change the following lines in `main.rs` and recompile

```
// number_bins: The number of lines in the data file (10000)
let number_bins: u64 = 200000;
// number_measure: How many measurements to average over per bin (500)
let number_measure: u64 = 500;
// number_update: How many updated before a measurement (5)
let number_update: u64 = 5;
```
