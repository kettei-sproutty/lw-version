# lw-version (WIP)

`Versioning tool` based on [Semantic Versioning](https://semver.org) for NodeJS projects..

<img src="images/logo.webp" width="1200"  alt="Logo">

[![Test](https://github.com/kettei-sproutty/lw-version/actions/workflows/test.yml/badge.svg)](https://github.com/kettei-sproutty/lw-version/actions/workflows/test.yml)

## Usage

lw-version is a command-line tool that can be used to generate [semantic version numbers](https://semver.org/#summary).

To use lw-version, simply pass in the desired version number as an argument.

## Commands

### Major
To generate a major version number, you would use the following command:

```bash
lw-version major
```

### Minor
To generate a minor version number, you would use the following command:

```bash
lw-version minor
```

### Patch
To generate a patch version number, you would use the following command:

```bash
lw-version patch
```

## Ci / Auto
This command analyzes the commit history of the current branch and generates a semantic version number.

```bash
lw-version ci
```

## License

`lw-version` is licensed under the MIT License.
