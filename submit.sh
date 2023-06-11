#!/bin/bash

set -eu

CONTEST_ID="" # e.g., ahc020

if [[ $CONTEST_ID == "" ]]; then
    echo "Error: Update CONTEST_ID in this script"
    exit 1
fi

cargo check

tmpfile=$(mktemp /tmp/submission.XXXXXX.rs)

echo "Writing submission file to $tmpfile"
cargo equip --bin submission --exclude-atcoder-crates --no-check > $tmpfile

echo "Submitting to $CONTEST_ID"
oj submit "https://atcoder.jp/contests/${CONTEST_ID}/tasks/${CONTEST_ID}_a" $tmpfile -y


