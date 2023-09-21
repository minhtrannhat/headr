# Rust rewrite of BSD head

``` text
headr 0.1.0
    Minh Tran <minh@minhtrannhat.com>
Rust rewrite of BSD headr

Usage: headr [OPTIONS] [FILE]...
Arguments:
  [FILE]...  Input file(s) [default: -]

Options:
  -n, --lines <lines>  Print the first K lines instead of the first 10 with the leading '-', print all but the last K lines of each file [default: 10]
  -c, --bytes <bytes>  print the first K bytes of each file; with the leading '-', print all but the last K lines of each file
  -h, --help           Print help
  -V, --version        Print version
```

## Build
`cargo build`

## Test
`./mk-outs.sh && cargo test`
