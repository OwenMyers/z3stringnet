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
target/debug/z3stringnet --size <int> --weights <float> --nbins 200000 \
   --nmeasure 500 --nupdate 5 --loop-update
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

# Results

## Configuration Outputs

### Vertex Configurations
Link orientations are specified with respect to the **vertex**..

Both sublattices are now written when the configuration output flag is set.

### Plaquette Configurations

A plaquette view is also written. In the plaquett configurations 
links are specified using absolute orientations. 
An example of a plaquett view header and first line is:

```
x,y,N,E,S,W
0,0,E,N,B,B
```

The columns are

* `x`: horizontal position of plaquette
* `y`: vertical position of plaquette
* `N`: The top link of the plaquette
* `E`: The right link 
* `S`: The botom link
* `W`: The left link

The values in the example are 
* `x`: located at the `x=0` position
* `y` located at the `y=0` position
* The top link is pointing to the right
* The right link is pointing up
* The bottom link is blank
* The left link is blank
