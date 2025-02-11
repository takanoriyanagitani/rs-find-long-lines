# Simple Benchmark

## macOS

input: enwiki-20231101-pages-articles-multistream-index.txt

| tool                            | elapsed | rate     | ratio |
|:-------------------------------:|:-------:|:--------:|:-----:|
| rs-find-long-lines(native)      | 1.5s    | 699 MB/s | 12x   |
| rs-find-long-lines(wasi/wazero) | 1.9s    | 540 MB/s | 10x   |
| awk                             | 19.4s   |  57 MB/s | (1x)  |
