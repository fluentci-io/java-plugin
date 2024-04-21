# Java Plugin

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

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    pipeline: .
    args: |
      setup zulu-17.46.19
    working-directory: example
- name: Show Java version
  run: |
    export PATH=${HOME}/.local/share/mise/shims:${PATH}
    type java
    java -version
```
