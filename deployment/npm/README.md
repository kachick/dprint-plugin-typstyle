# @dprint/typstyle

npm distribution of [dprint-plugin-typstyle](https://github.com/dprint/dprint-plugin-typstyle) which is an adapter plugin for [typstyle](https://github.com/typstyle-rs/typstyle).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-typstyle Wasm file](https://github.com/dprint/dprint-plugin-typstyle/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@dprint/typstyle";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText({
  filePath: "file.typ",
  fileText: "#set text(  size: 10pt  )",
}));
```
