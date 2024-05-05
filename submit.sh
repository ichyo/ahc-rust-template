#!/bin/bash

set -eu

CONTEST_ID="" # e.g., ahc020
TASK_ID="$CONTEST_ID"_a

if [[ $CONTEST_ID == "" ]]; then
    echo "Error: Update CONTEST_ID in this script"
    exit 1
fi

cargo check

tmpfile=$(mktemp /tmp/submission.XXXXXX.rs)

echo "Writing submission file to $tmpfile"
./generate.sh > $tmpfile

echo "Submitting to $CONTEST_ID"
oj submit "https://atcoder.jp/contests/${CONTEST_ID}/tasks/${TASK_ID}" $tmpfile -y



