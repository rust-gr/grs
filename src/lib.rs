#![warn(unsafe_op_in_unsafe_fn)]

//! # GR crate
//!
//! The GR crate provides a wrapper for the GR framework, which is a library written in C.
//!
//! ## GR - a universal framework for visualization applications
//!
//! *GR* is essentially based on an implementation of a Graphical Kernel System (GKS)
//! and OpenGL. As a self-contained system it can quickly and easily be integrated
//! into existing applications.
//!
//! The *GR* framework can be used in imperative programming systems or integrated
//! into modern object-oriented systems, in particular those based on GUI toolkits.
//! *GR* is characterized by its high interoperability and can be used with modern
//! web technologies. The *GR* framework is especially suitable for real-time
//! or signal processing environments.
//!
//! *GR* was developed by the Scientific IT-Systems group at the Peter Grünberg
//! Institute at Forschunsgzentrum Jülich. The main development has been done
//! by Josef Heinen who currently maintains the software, but there are other
//! developers who currently make valuable contributions. Special thanks to
//! Florian Rhiem (*GR3*) and Christian Felder (qtgr, setup.py).
//!
//! For further information please refer to the [GR home page](http://gr-framework.org).
//!
//! ## Examples
//! This program draws a simple line on screen.
//! There is a similar example shipped with this crate: `gks-simple`.
//! ```
//! # use std::error::Error;
//! # use gr::gr::*;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! polyline(2, &[0.0, 1.0], &[0.0, 1.0])?;
//! updatews();
//! #   Ok(())
//! # }
//! ```

pub use gr_sys as ffi;
pub mod gks;
pub mod gr;
pub mod util;
