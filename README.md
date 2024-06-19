# Java Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/java)](https://pkg.fluentci.io/java)
[![ci](https://github.com/fluentci-io/java-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/java-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of Java.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm java setup zulu-17.46.19 # or any other version
```

## Functions

| Name  | Description                          |
| ----- | ------------------------------------ |
| setup | Installs a specific version of Java. |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/java@v0.1.2?wasm=1", "setup", vec!["zulu-17.46.19"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: java
    args: |
      setup zulu-17.46.19
- name: Show Java version
  run: |
    export PATH=${HOME}/.local/share/mise/shims:${PATH}
    type java
    java -version
```
