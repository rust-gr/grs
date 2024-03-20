use super::util::impl_primitive_function;
use core::ffi::c_int;
use gr_sys::gr::*;
use paste::paste;

// Segments
impl_primitive_function! { createseg, () {segment, c_int} }
impl_primitive_function! { copysegws, () {segment, c_int} }
impl_primitive_function! { redrawsegws, () }
impl_primitive_function! { setsegtran, () {segment, c_int}, {fx, f64}, {fy, f64}, {transx, f64}, {transy, f64}, {phi, f64}, {scalex, f64}, {scaley, f64} }
impl_primitive_function! { closeseg, () }
