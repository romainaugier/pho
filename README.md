# pho

pho (Perfect Hash Oracle) is a cli tool that generates perfect hash-tables for any given dataset, in many languages.

## Usage

Usage: `pho [OPTIONS] --file <FILE> --name <NAME>`

Options:
  - -f, --file <FILE>                            
  - -k, --key-type <KEY_TYPE>                [default: string]
  - -o, --output <OUTPUT>                    [default: pho_output.c]
  - -n, --name <NAME>                            
  - --namespace <NAMESPACE>                  [default: pho]
  - --first-order-hash <FIRST_ORDER_HASH>    [default: murmur3]
  - --second-order-hash <SECOND_ORDER_HASH>  [default: xorshift]
  - -h, --help                               Print help
  - -V, --version                            Print version

## Features

Supported languages for code generation are:
  - C
  - Python
  
Supported first-order hash functions:
  - fnv1a
  - xxhash32
  - murmur3_32
  
Supported second-order hash functions:
  - mxf (Multiply Xor Fold)
  - Wang-hash and Xorshift
  
## Known Issues

This tool is a work-in-progress and has limits (for now) due to only supporting 32-bits hash functions. The english dictionary from the test_data has 10-15 hard-collisions but in the close future 64-bits hashes will be implemented. More languages for code-generation are on the way too.
