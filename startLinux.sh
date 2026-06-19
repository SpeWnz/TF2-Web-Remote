#!/bin/bash
python3 -m venv venv

source ./venv/bin/activate

./venv/bin/pip install --upgrade pip

if pip install -r ./requirements.txt; then
    echo 'Dependencies OK.'
    ./venv/bin/python3 main.py
else
    echo 'Error fetching dependencies.'
    exit 1
fi