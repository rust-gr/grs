#!/bin/bash

# Reasons against rust-bindgen as a build-dependency:
#   - no real upside due to lack of architecture-specific definitions
#   - rust-bindgen carries many dependencies (including libclang)
#   - updating manually prevents bugs
${BINDGEN:-bindgen} --use-core --no-layout-tests gr/lib/gks/gks.h -o src/gks/bindings/gks.rs
${BINDGEN:-bindgen} --use-core --no-layout-tests --blocklist-type 'size_t' gr/lib/gks/gkscore.h -o src/gks/bindings/gkscore.rs

cat <<- EOF >> src/gks/bindings/gks.rs
	pub unsafe fn gsetlinecolorind(x: Gint) -> ::core::ffi::c_int { gsetlinecolourind(x) }
	pub unsafe fn gsetmarkercolorind(x: Gint) -> ::core::ffi::c_int { gsetmarkercolourind(x) }
	pub unsafe fn gsettextcolorind(x: Gint) -> ::core::ffi::c_int { gsettextcolourind(x) }
	pub unsafe fn gsetfillcolorind(x: Gint) -> ::core::ffi::c_int { gsetfillcolourind(x) }
	pub unsafe fn gsetcolorrep(x: Gint, y: Gint, p: *mut Gcobundl) -> ::core::ffi::c_int { gsetcolourrep(x, y, p) }
	pub unsafe fn ginqlinecolorind(a: *mut Gint, b: *mut Gint) -> ::core::ffi::c_int { ginqlinecolourind(a, b) }
	pub unsafe fn ginqtextcolorind(a: *mut Gint, b: *mut Gint) -> ::core::ffi::c_int { ginqtextcolourind(a, b) }
	pub unsafe fn ginqmarkercolorind(a: *mut Gint, b: *mut Gint) -> ::core::ffi::c_int { ginqmarkercolourind(a, b) }
EOF
cat <<- EOF >> src/gks/bindings/gkscore.rs
	#[allow(non_snake_case)]
	pub fn FIX_COLORIND(c: u32) -> u32 { if c < MAX_COLOR { c } else { MAX_COLOR - 1 } }
EOF
