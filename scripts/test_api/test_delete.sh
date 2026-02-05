#!/bin/bash

echo "arguments :"
echo "1: email"

_count=0
for _var in "$1"; 
do
  if [ -z "$_var" ]; then
    ((_count++)) &&
    echo "The param $_count is missing" &&
    exit 1
  fi
done

curl -i -X DELETE -H "content-type: application/json" -d "{\"email\": \"$1\"}" http://localhost:3000/users
curl http://localhost:3000/users
