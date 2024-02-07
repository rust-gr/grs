#!/bin/bash

main() {
	rm header/*
	echo -e "#![allow(warnings)]\n" > src/lib.rs
	local h
	for h in gks/gks.h gks/gkscore.h gr/gr.h
	do
		local base name header url patch
		base=${h##*/}
		name=${base%.h}
		header=header/${base}
		url=https://raw.githubusercontent.com/sciapp/gr/develop/lib/$h
		patch=patches/${name}.patch
		curl -o ${header} ${url} &&
		if [ -f ${patch} ]
		then
			# patch can successfully execute with exit-code 0 even though the patch file is faulty
			# it generates a .orig file so that's how you can tell
			patch ${header} < ${patch} && ! { ls header | grep -q '.orig' && echo not quite right; }
		fi || return
		echo -e "pub mod ${name} { include!(concat!(env!(\"OUT_DIR\"), \"/${name}.rs\")); }\n" >> src/lib.rs
	done
	cargo fmt
	cargo clean
	cargo b --release --features bindgen
	rm -rf bindings
	mkdir bindings
	cp target/release/build/gr-sys-*/out/* bindings
}

main $@
