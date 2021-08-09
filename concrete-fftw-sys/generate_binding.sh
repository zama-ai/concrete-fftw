#!/usr/bin/env bash

set -e

CONCRETE_DIR=$(dirname "$0")
FFTW_DIR="$CONCRETE_DIR/fftw-3.3.8"

cd $FFTW_DIR
./configure --with-pic --enable-static --enable-avx --enable-avx2 --enable-sse2 \
--enable-generic-simd128 --enable-generic-simd256 --disable-doc --prefix $FFTW_DIR
make -j16
make install

./configure --with-pic --enable-static --enable-single --enable-avx --enable-avx2 --enable-sse \
--enable-sse2 --enable-generic-simd128 --enable-generic-simd256 --disable-doc --prefix $FFTW_DIR
make -j16
make install

bindgen \
  --use-core \
  --with-derive-{default,eq,hash,ord} \
  --whitelist-type="^fftw.*" \
  --whitelist-var="^FFTW.*" \
  --whitelist-function="^fftw.*" \
  --blacklist-type="FILE" \
  --blacklist-type="_.*" \
  --blacklist-type="fftw.*_complex" \
  --blacklist-function="fftw.*wisdom.*" \
  --blacklist-function="fftw.*fprint.*" \
  --blacklist-function="fftwl_.*" \
  --default-enum-style=rust \
    include/fftw3.h \
  > ../src/fftw.rs