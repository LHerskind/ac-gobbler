# Aztec Connect Data Gobbler

The Aztec Connect Data gobbler is a tool made for extracting data from the Aztec Connect system using only L1 as its
source. The tool looks for new rollup blocks being published, and then pulls and decodes them.

The decoded blocks are stored in a simple key-value store (`MicroKV`), and the transactions can be exported to a
CSV-file such that it can easily be passed into Python or similar tools.

## Usage

The tool is written in Rust, and can be compiled using `cargo build --release`. The compiled binary will be
located in `target/release/aztec-data-gobbler`. The key-value store is written to disk for later use, and is saved in
the directory passed in to `--data-path` or use the default.

### Syncing

To sync the node, you need point it towards an L1 node (can be Infura) and can then sync it using

```bash
Synchronise the local database with the rollup

Usage: ac-l1-gobbler sync [OPTIONS]

Options:
      --rpc-url <RPC_URL>      The RPC url to an ethereum node [default: http://localhost:8545]
      --data-path <DATA_PATH>  The path to the dir of the database [default: ./data/]
  -h, --help                   Print help
```

### Exporting

Syncing might take a while, as the tool needs to download all the rollup blocks. Once it is done, rerunning will be much
faster as it will only download new blocks.

The tool can also export the data to a CSV-file, which can be used in Python or similar tools. To do this, run

```bash
Exports all transactions to a csv file

Usage: ac-l1-gobbler export [OPTIONS]

Options:
      --export-path <EXPORT_PATH>  The file to write csv to [default: ./export/txs.csv]
  -l, --l1-only                    Export only deposits and withdrawals
      --data-path <DATA_PATH>      The path to the dir of the database [default: ./data/]
  -h, --help                       Print help
```

### Decoding individual blocks

Individual blocks can also be decoded using the `decode` command, which can be useful for getting an understanding of
what is going on in a specific block.

```bash
Print the contents of a block in a semi readable manner

Usage: ac-l1-gobbler decode [OPTIONS] <ROLLUP_ID>

Arguments:
  <ROLLUP_ID>  The rollup id of the block to decode

Options:
      --data-path <DATA_PATH>  The path to the dir of the database [default: ./data/]
  -h, --help                   Print help
```


