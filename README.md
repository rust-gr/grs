GR - a universal framework for visualization applications
=========================================================

[![Screenshots](https://gr-framework.org/_images/screenshots.png)](https://gr-framework.org)

*GR* is essentially based on an implementation of a Graphical Kernel System (GKS)
and OpenGL. As a self-contained system it can quickly and easily be integrated
into existing applications.

The *GR* framework can be used in imperative programming systems or integrated
into modern object-oriented systems, in particular those based on GUI toolkits.
*GR* is characterized by its high interoperability and can be used with modern
web technologies. The *GR* framework is especially suitable for real-time
or signal processing environments.

*GR* was developed by the Scientific IT-Systems group at the Peter Grünberg
Institute at Forschunsgzentrum Jülich. The main development has been done
by Josef Heinen who currently maintains the software, but there are other
developers who currently make valuable contributions. Special thanks to
Florian Rhiem (*GR3*) and Christian Felder (qtgr, setup.py).

For further information please refer to the [GR home page](http://gr-framework.org).

Using the *GR* Rust-crate
--------------------------

1. [Install *GR*](https://gr-framework.org/c.html#installation)
1. Set the `GRLIB` environment variable to the path of *GR*'s `lib` directory / folder
    - It should contain `.lib`, `.dylib` or `.so` files
    - On Windows (and also MinGW) you should use an absolute path
    - On Linux and Mac some paths will be searched when `GRLIB` doesn't refer to a usable installation:
        - `~/gr` (ie. implicit `GRLIB=~/gr/lib`)
        - `/usr/gr` (ie. implicit `GRLIB=/usr/gr/lib`)
        - `/usr/local/gr` (ie. implicit `GRLIB=/usr/local/gr/lib`)
    - This variable is only needed at compile-time
1. The crate's build-script outputs a path as `cargo:lib_dir`
    - The Rust program will need to load libraries from that path!
      **Make it accessible!**\
      You may want to put this into your `PATH` or your binaries' rpaths.
    - On Windows this is **not** the same as `GRLIB`!\
      It will point to the `bin` folder, which contains the `.dll`s.

To test your setup, try compiling and running this simple application:
```rs
use ::gr::gr;
use std::io::{stdin, Read};

fn main() {
    gr::polyline(2, &[0.0, 1.0], &[0.0, 1.0]).unwrap();
    gr::updatews();
    stdin().bytes().next();
}
```

License
-------

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
