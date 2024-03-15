import * as wasm from "../pkg/index_bg.wasm";
import { __wbg_set_wasm } from "../pkg/index_bg.js";

__wbg_set_wasm(wasm);

export * from "../pkg/index_bg.js";

wasm.__wbindgen_start();
