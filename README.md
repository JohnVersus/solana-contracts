### `Solana Smart Contracts`

`cd` to the rust project which you want to use.

```bash
cd folder_name
```

Build the Solana Rust Program using

```bash
cargo build-bpf
```

Once built successfully without any error `.so` of the program will be added to the `/target/deploy` folder. You can deploy this to solana cluster using.

```bash
solana program deploy ./target/deploy/file_name.so
```

Once successfully deployed it will return the programId of the Solana Program.
