# dprint-plugin-vue

Format Vue SFC.

This plugin format root-level blocks through `dprint`, meaning you will need to
install plugins for the languages contained in your Vue SFCs.

## Language detection

Unless a `lang` attribute is present:

| Block      | Default |
| ---------- | ------- |
| `script`   | `js`    |
| `template` | `html`  |
| `style`    | `css`   |

## Usage

[Install](https://dprint.dev/install) and [setup](https://dprint.dev/setup)
dprint, then:

1. Run
   ```shell
   dprint config add malobre/vue
   ```
2. Install plugins for the languages contained in your vue files.
3. Ensure `.vue` file extensions are matched in an `includes` pattern:
   ```jsonc
   {
     // -- snip --
     "includes": [
       "**/*.vue"
     ]
   }
   ```
4. Add a `vue` configuration property if desired:
   ```jsonc
   {
     // -- snip --
     "vue": {
       // vue config goes here
     }
   }
   ```

## Configuration

| Key              | Default | Description                                |
| ---------------- | ------- | ------------------------------------------ |
| `indentTemplate` | `true`  | Indent the content of the `<template>` tag |
| `indentWidth`    | `2`     | Width of the indentation                   |
| `useTabs`        | `false` | Use tabs for indentation                   |

## Building

Run

```shell
cargo build --release --target=wasm32-unknown-unknown
```
