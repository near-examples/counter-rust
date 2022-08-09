#!/bin/bash
TEXT=$(timeout 150 yarn start) 
if [[ ! "$TEXT" =~ .*"Server running at http://localhost:1234".* ]]; then 
    exit 1 
else 
    exit 0 
fi