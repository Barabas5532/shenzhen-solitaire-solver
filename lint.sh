#!/usr/bin/env bash

files=`echo *.py **/*.py`

echo $files

python -m black $files
python -m flake8 $files
python -m mypy --disallow-untyped-defs --check-untyped-defs --disallow-untyped-calls $files
