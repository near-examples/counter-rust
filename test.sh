#!/bin/bash

# This script is for use with Linux/macOS/Unix.
# It was auto-generated by near-workspaces-ava,
# and only runs the tests in the `near-workspaces` folder.
cd near-workspaces
npm install
npm run test --verbse $@ # pass along any CLI flags, such as `--verbose`