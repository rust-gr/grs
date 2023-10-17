#!/bin/sh
curl -o header/gks.h https://raw.githubusercontent.com/sciapp/gr/master/lib/gks/gks.h
patch header/gks.h < patches/gks.patch
curl -o header/gkscore.h https://raw.githubusercontent.com/sciapp/gr/master/lib/gks/gkscore.h
patch header/gkscore.h < patches/gkscore.patch
