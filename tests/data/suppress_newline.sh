#!/bin/sh

set -eu

echo '!' >suppress_newline.txt
prepend -n suppress_newline.txt 'hello, world'
