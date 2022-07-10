#!/usr/bin/env bash
python -m black *.py
python -m flake8 *.py
python -m mypy --disallow-untyped-defs --check-untyped-defs --disallow-untyped-calls *.py
