#!/bin/sh

set -eu

echo bye >basic.txt
prepend basic.txt hello world
