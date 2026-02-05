#!/bin/bash

sea-orm-cli generate entity \
    -o src/modules/models/entities \
    --with-serde both \
    --lib