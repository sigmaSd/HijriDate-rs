# Deno bindings for wasm

## Usage

```ts
import { HijriDate } from "https://github.com/sigmaSd/HijriDate-rs/raw/master/deno/pkg/hijri_date.js";

const hijriDate = HijriDate.today();
console.log(hijriDate.toString());
```

## Developement

```
deno task build # To update wasm bindings, requires $CARGO_TARGET_DIR set
deno run -A example.ts # example
deno test -A # test
```
