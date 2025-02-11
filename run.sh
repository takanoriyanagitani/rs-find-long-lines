#!/bin/sh

echo helo | ENV_MIN_LEN_INCLUSIVE=0 ./rs-find-long-lines
echo hel1 | ENV_MIN_LEN_INCLUSIVE=1 ./rs-find-long-lines
echo hel2 | ENV_MIN_LEN_INCLUSIVE=2 ./rs-find-long-lines
echo hel3 | ENV_MIN_LEN_INCLUSIVE=3 ./rs-find-long-lines
echo hel4 | ENV_MIN_LEN_INCLUSIVE=4 ./rs-find-long-lines
echo hel5 | ENV_MIN_LEN_INCLUSIVE=5 ./rs-find-long-lines

echo helo5 | ENV_MIN_LEN_INCLUSIVE=5 ./rs-find-long-lines
