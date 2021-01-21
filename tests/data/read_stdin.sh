#!/bin/sh

set -eu

echo END >read_stdin.txt
(
    sleep 0.1s; echo 1
    sleep 0.1s; echo 2
    sleep 0.1s; echo 3
) | prepend read_stdin.txt -
