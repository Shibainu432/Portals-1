#!/bin/sh

set -ex

wasm-pack build --target web
rm pkg/.gitignore   # I need the code for GitHub Pages