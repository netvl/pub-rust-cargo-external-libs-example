#!/bin/bash

cd foo

rm libfoo.a foo.o

gcc -c foo.c -fPIC
ar qf libfoo.a foo.o

cp libfoo.a "$OUT_DIR"
