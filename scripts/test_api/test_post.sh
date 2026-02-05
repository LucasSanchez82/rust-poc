#!/bin/bash

echo "arguments :"
echo "1: email"
echo "2: name"
echo "3: password"

_count=0
for _var in "$1" "$2" "$3"; 
do
  if [ -z "$_var" ]; then
    ((_count++)) &&
    echo "The param $_count is missing" &&
    exit 1
  fi
done

curl -X POST -H "content-type: application/json" -d "{\"email\": \"$1\", \"name\": \"toto\", \"password\": \"$3\"}" http://localhost:3000/users
curl http://localhost:3000/users
