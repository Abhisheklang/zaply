# Cli

## Testing

You can test your changes to the `cli` crate by first building the main zaply binary:

```
cargo build -p zaply
```

And then building and running the `cli` crate with the following parameters:

```
 cargo run -p cli -- --zaply ./target/debug/zaply.exe
```
