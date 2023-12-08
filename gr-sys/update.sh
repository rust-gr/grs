#!/bin/bash
for h in gks/gks.h gks/gkscore.h
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
done
