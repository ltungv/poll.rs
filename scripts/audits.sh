#!/bin/bash

set -euxo pipefail

cargo deny check \
&& cargo outdated --exit-code 1 \
&& rm -rf ~/.cargo/advisory-db \
&& cargo audit \
&& cargo pants \
&& echo "OK!"
