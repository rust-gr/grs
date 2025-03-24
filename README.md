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
1. In your Rust project, issue the command ``cargo add gr``
1. Set the `GRLIB` environment variable to the path of *GR*'s `lib` directory / folder
    - It should contain `.lib`, `.dylib` or `.so` files
    - On Windows (and also MinGW) you should use an absolute path
    - On Linux and Mac some paths will be searched when `GRLIB` doesn't refer to a usable installation:
        - `~/gr` (ie. implicit `GRLIB=~/gr/lib`)
        - `/usr/gr` (ie. implicit `GRLIB=/usr/gr/lib`)
        - `/usr/local/gr` (ie. implicit `GRLIB=/usr/local/gr/lib`)
    - This variable is only needed at compile-time
1. Your Rust program will need to dynamically load *GR* libraries! **Make them accessible!**
    - Ensure *GR*'s library files are found by your OS's dynamic loader.\
      If that's already the case, you don't have to do anything.
    - You may need to edit your `PATH` (Windows), `LD_LIBRARY_PATH` (Linux), `DYLD_LIBRARY_PATH` (Mac) or your binaries' rpaths.
    - On Windows, the required path is **not** the same as `GRLIB`!\
      It should point to the `bin` folder, which contains the `.dll`s.\
      Alternatively, you can just copy the `.dll`s next to your `.exe`s.
    - On Linux and Mac, you can set the rpath when building an executable from a Rust crate like this:
      ```sh
      RUSTFLAGS="-C link-arg=-Wl,-rpath,/my/path/to/gr/lib" cargo build
      ./target/debug/mybinaryname  # or run it using `cargo run` above
      ```
      This is useful if *GR* isn't found normally, but the resulting binary shouldn't be distributed.

To test your setup, try compiling and running this simple application:
```rs
use gr::gr;
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
