# lindera-js

[![npm version](https://badge.fury.io/js/lindera-js.svg)](https://badge.fury.io/js/lindera-js)

A [lidera](https://github.com/lindera-morphology/lindera) japanese tokenizer wrapper for javascript and typescript

# Usage

```bash
npm i lindera-js
```

```javascript
import * as lindera from "lindera-js";

console.log(lindera.tokenize("関西国際空港限定トートバッグ"));
```

If you want to see more detail usage, please show examples/hello-lindera-js.

# build

```bash
wasm-pack build
```

# test

```bash
wasm-pack test --node
```