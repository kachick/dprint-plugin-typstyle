// @ts-check
const assert = require("assert");
const createFromBuffer = require("@dprint/formatter").createFromBuffer;
const getBuffer = require("./index").getBuffer;

const formatter = createFromBuffer(getBuffer());
const result = formatter.formatText({
  filePath: "file.typ",
  fileText: "#set text(  size: 10pt  )",
});

assert.strictEqual(result, "#set text(size: 10pt)\n");
