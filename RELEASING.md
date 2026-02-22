# Releasing

## Checklist

1. Update dependencies:

```sh
cargo update --workspace
```

2. Run the tests:

```sh
cargo test
```

3. Update the version number in `Cargo.toml`.

4. Review the man page (verify the version number matches `Cargo.toml`):

```sh
cargo build
make show-man
```

5. Commit and merge the version bump on a branch, then tag on `main`:

```sh
git add -A && git commit -m "<version>"
```

After the branch is merged:

```sh
git tag <version>
git push --tags
```

6. Publish to [crates.io](https://crates.io):

```sh
cargo publish
```

7. Update the Homebrew formula in the `homebrew-tap` repository.
