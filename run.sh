#!/bin/bash

option=$1

if [[ $option != "-no-venv" ]]; then
    source .venv/bin/activate
fi

maturin develop
python main.py
