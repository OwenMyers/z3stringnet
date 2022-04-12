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

# Additional Steps as of 2022-04 
## (Ubuntu)
I recently added a GUI for visualizing estimators.
You don't need to use the GUI to run, but there are some additional dependencies because of it.

Make sure you have up-to-date apt: 
```
sudo apt update
```

and make sure you have gcc
```
sudo apt install gcc
```

When you compile (see below section) you might get an error that looks like:

```
  = note: /usr/bin/ld: cannot find -lxcb
          /usr/bin/ld: cannot find -lxcb-render
          /usr/bin/ld: cannot find -lxcb-shape
          /usr/bin/ld: cannot find -lxcb-xfixes
          collect2: error: ld returned 1 exit status
```

In which case you will need:

```
sudo apt install libxcb-xfixes0-dev
```


# Compile
```
git clone https://github.com/OwenMyers/z3stringnet.git
```

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
./target/debug/z3stringnet 
    --size <lattice edge length>
    --weights 0.5 
    --nbins <number of bins>
    --nmeasure <number of measurements per bin>
    --nupdate <number of updates per measurement asdfsd 
    --write-update-confs false
    --loop-update
    --gui false
```

A working example:
```
./target/debug/z3stringnet 
    --size 4
    --weights 0.5 
    --nbins 10
    --nmeasure 5
    --nupdate 100
    --write-update-confs false
    --loop-update
    --gui false
```

For some more detail on each of the flags:
```
./target/debug/z3stringnet --help
```

Or you can find the same information in human readable form in `src/cli.yml`

If you compiled with the `--release` flag and want to run fast

```
target/release/z3stringnet <flags>
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

# Conrod

You need to `apt` install `libxcb-composite0-dev`
