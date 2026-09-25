# @kachick/dprint-plugin-typstyle

npm distribution of [dprint-plugin-typstyle](https://github.com/kachick/dprint-plugin-typstyle).

## dprint Configuration

Add the plugin to your `dprint.json`:

```json
{
  "plugins": [
    "npm:@kachick/dprint-plugin-typstyle@0.5.0"
  ]
}
```

Or install with your package manager:

```sh
npm install --save-dev @kachick/dprint-plugin-typstyle
```

and add without a version:

```json
{
  "plugins": [
    "npm:@kachick/dprint-plugin-typstyle"
  ]
}
```

## JavaScript API

Use with [@dprint/formatter](https://github.com/dprint/js-formatter):

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getPath } from "@kachick/dprint-plugin-typstyle";
import * as fs from "node:fs";

const buffer = fs.readFileSync(getPath());
const formatter = createFromBuffer(buffer);

console.log(formatter.formatText("example.typ", "=   Introduction  \n"));
```
