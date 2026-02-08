#!/bin/bash

sea-orm-cli generate entity \
    -o src/modules/models/entities \
    --entity-format dense \
    --with-serde both \
    --lib # \
    # --big-integer-type i64 \
    