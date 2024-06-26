# TestEZ Companion CLI

> [!WARNING]
> This isn't well tested and was made in a couple hours. If something doesn't work, please [file an issue](https://github.com/jackTabsCode/testez-companion-cli/issues).

This is a quick and dirty CLI to interface with [TestEZ Companion](https://github.com/tacheometry/testez-companion). It was made to allow users to run tests from the command line without needing to use the VSCode extension.

You'll need the plugin for this to work, which is [bundled in the latest release](https://github.com/jackTabsCode/testez-companion-cli/releases/latest/download/TestEZ_Companion.rbxm) (alternatively, you can [build it from source](https://github.com/tacheometry/testez-companion/tree/main/plugin)).

![SCR-20240421-cxmd](https://github.com/jackTabsCode/testez-companion-cli/assets/44332148/246a6cd6-5b65-47a1-8c74-9baa7487448e)

## Features

-   Easily run tests from the command line
-   Supports multiple places
-   Pretty prints results
-   Prints logs and other output from Studio
-   ~~Installs the plugin for you~~ (not implemented yet)

## Installation

### Aftman

```sh
aftman add jacktabscode/testez-companion-cli
```

### Cargo

```sh
cargo install testez-companion-cli
```

## Usage

Just run `testez-companion-cli` in your terminal. If there are multiple places open, it will prompt you to select one. It will then output the results of the tests!

If you wanted, you could add this to a pre-commit hook to ensure that tests pass before committing (though, it would require your place be open in Studio, of course).
