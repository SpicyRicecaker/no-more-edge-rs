# no-more-edge-rs

Replaces calls to microsoft edge with calls to your default browser on windows. Inspired by the C# project [NoMoreEdge](https://github.com/HarshalKudale/NoMoreEdge).

## Installation

Simply download and run the `.msi` installer in releases. 

### Uninstallation

Uninstall the program as you would a regular windows program in control
panel. This program basically only registers a single registry key so it's
just a matter of deleting that key.

## Building Manually

This project uses [cargo-wix](https://github.com/volks73/cargo-wix) to build the `.msi` installer for the app and write the necessary registry key. Install it via 

```shell
cargo install cargo-wix
```

Then simply run 

```shell
cargo wix
```

The installer should then be located at `target\wix\no-more-edge-rs-x.x.x-x86_64.msi`
