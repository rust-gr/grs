#!/bin/bash
rm header/*
echo -e "#![allow(warnings)]\n" > src/lib.rs
for h in gks/gks.h gks/gkscore.h gr/gr.h
do
	base=${h##*/}
	name=${base%.h}
	header=header/$base
	patch=patches/$name.patch
	curl -o $header https://raw.githubusercontent.com/sciapp/gr/master/lib/$h
	if [ -f $patch ]
	then
		patch $header < $patch
	fi
	echo -e "pub mod $name { include!(concat!(env!(\"OUT_DIR\"), \"/$name.rs\")); }\n" >> src/lib.rs
done
cargo fmt
cargo clean
cargo b --features bindgen
rm -rf bindings
mkdir bindings
cp target/debug/build/gr-sys-*/out/* bindings
