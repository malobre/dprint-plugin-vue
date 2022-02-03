# dprint-plugin-vue

Format Vue SFC.

This plugin only parses root-level blocks, extract their content, and format
them through `dprint` according to the `lang` attribute or a default:

| Block      | Default |
| ---------- | ------- |
| `script`   | `js`    |
| `template` | `html`  |
| `style`    | `css`   |

This means you also need to install plugins for languages contained in your Vue SFC, such as
[`dprint-plugin-typescript`] for JavaScript / TypeScript.

[`dprint-plugin-typescript`]: https://github.com/dprint/dprint-plugin-typescript

## Usage

[Install](https://dprint.dev/install) and [setup](https://dprint.dev/setup) dprint, then:

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
