#!/bin/sh

set -eu

echo -e 'orange:\n\t\\120' >escape.txt
prepend -e escape.txt 'apple:\n\t\\100'
