# nw

A tiny CLI that finds the command runner used in the current project and runs it with the passed arguments.

So instead of looking up if a project uses pnpm, Yarn, or npm, you can just run `nw test`, and it will delegate the command to the correct runner.

Here's the list of supported runners:

| Runner | Command         |
| ------ | --------------- |
| Just   | `just ARGS`     |
| Make   | `make ARGS`     |
| npm    | `npm run ARGS`  |
| pnpm   | `pnpm run ARGS` |
| Yarn   | `yarn run ARGS` |
| cargo  | `cargo ARGS`    |

## Getting started

### Installation

To install `nw`, you can use `cargo`:

```sh
cargo install nw
```

## Changelog

See [the changelog](./CHANGELOG.md).

## License

[MIT © Sasha Koss](https://kossnocorp.mit-license.org/)
