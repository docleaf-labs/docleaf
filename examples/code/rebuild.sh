#!/bin/bash -e

for dir in $(ls -d1 */); do
  echo $dir
  pushd $dir
    rm -fr xml
    doxygen
  popd
done
