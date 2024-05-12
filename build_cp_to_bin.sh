#!/bin/bash
#
cargo build --release && 
  cp -a target/release/acty ~/.local/bin/acty
