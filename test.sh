#!/bin/bash
# Usage: ./test.sh test_file_name_without_rs_extension
# Example: ./test.sh ads1115_read

# laptop@host:~$ ls -l /dev/i2c-6
# crw-rw---- 1 root i2c 89, 6 Jun  9 19:18 /dev/i2c-6
# laptop@host:~$ sudo usermod -aG i2c $USER
# [sudo] password for laptop:
# Sorry, try again.
# [sudo] password for laptop:

if [ -z "$1" ];
then
    echo "Usage: $0 test_file_name (without .rs extension)"
    exit 1
fi

TEST_NAME="$1"

# --- Test --------------------------------------------------------------------
echo "Running test file: $TEST_NAME"
RUST_LOG=trace RUST_BACKTRACE=1 cargo test --test "$TEST_NAME" -- --nocapture
