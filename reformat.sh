#!/bin/bash

cargo check && find . -name '*.rs' | xargs rustfmt && echo 'format complete'
