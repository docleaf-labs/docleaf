#!/bin/bash -e

# Sets up a virtual env in a temp directory to check that installing docleaf from
# pypi and running the documentation build works
# 
# We install Sphinx as we'd expect users to have Sphinx already

dir=$(mktemp --directory /tmp/docleaf-test-XXXXX)
echo Setting up in $dir

python3 -m venv $dir
source $dir/bin/activate

pip install docleaf

make -C documentation clean
make -C documentation html
