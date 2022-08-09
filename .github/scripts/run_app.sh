#!/bin/bash
TEXT=$(timeout 150 yarn start) 
if [[ ! "$TEXT" =~ .*"Server running at http://localhost:1234".* ]]; then 
    echo -e "\033[0;31m FAIL \033[0m"
    exit 1 
else 
    echo -e "\033[0;32m SUCCESS \033[0m"
    exit 0 
fi