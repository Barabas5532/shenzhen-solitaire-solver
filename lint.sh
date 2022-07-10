#!/usr/bin/env bash
python -m black solitaire.py
python -m flake8 solitaire.py
python -m mypy --disallow-untyped-defs --check-untyped-defs --disallow-untyped-calls solitaire.py
