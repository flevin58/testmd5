#!/bin/bash

if test "x$BINDGEN" == "x"; then
    BINDGEN=bindgen
fi
$BINDGEN --no-layout-tests --allowlist-function "md5" -o src/md5_bindings.rs vendor/md5/md5.c 
$BINDGEN --no-layout-tests --allowlist-function "sha256" -o src/sha256_bindings.rs vendor/sha256/sha256.h
