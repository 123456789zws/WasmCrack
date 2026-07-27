# WasmCrack
A toolsuite of advanced WASM static binary analysis tools designed to reverse engineer WebAssembly binaries.

> **Important Note:** WasmCrack is currently a work in progress. Several planned features are still in development.

**WasmCrack** is an advanced WebAssembly binary analysis toolkit designed specifically for reverse engineers. The ultimate vision for WasmCrack is to serve as a suite of utilities to help reverse engineers analyze, modify, and crack WebAssembly clients—particularly those used in complex browser-based web games.

---

## Getting Started

WasmCrack organizes your reverse engineering targets into **Projects**. 

### 1. Project Setup
To create a new project, add your target WebAssembly binary to the `binaries/` directory. The filename you provide (without the extension) will become your project name.

### 2. Execution
Once your binary is staged in the `binaries/` folder, initialize the project and run the analysis by executing:

```bash
cargo run <project_name_1> <project_name_2> ...
```

*Note: Executing a project will automatically run all currently available WasmCrack features against the target binary. If the output directories do not exist, WasmCrack will create them for you.*

### Example
Say we implement binary `test.wasm` into our binaries directory. To run the toolkit against this binary, we would run:

```bash
cargo run test
```

---

## Current Features

WasmCrack currently supports the following automated analysis utilities:

* **`wasm2js`** Parses a WebAssembly binary into a complete JavaScript interpretation. This is perfect for reverse engineers who need to read, understand, or replicate the underlying functionality of a Wasm module in a standard script environment. Output is dumped to `wasm2js-output.js` in your project directory. 
  
* **`code-magic`** Analyzes the binary's Code Section (Section 10) for heuristic patterns. It searches for hardcoded magic constants, identifies potential cryptographic signatures, and filters for UTF-8 encoded strings/fragments. Output is dumped to `code-magic-output.txt` in your project directory.
  
* **`data-magic`** Similar to `code-magic`, but specifically targets the binary's Data Section (Section 11) to extract sensitive constants, strings, and magic values. Output is dumped to `data-magic-output.txt` in your project directory.
  
* **`call-data`** Analyzes the Code Section and resolves data regarding func calls extending from each WebAssembly func. It also provides data on whether or not the execution for these calls is absolute (if the call is made at the root level, there is no branch logic stopping it from occurring), or if it is conditional/inconclusive (WasmCrack currently cannot safely determine if a call made in control flows in absolute). Output is dumped to `call-data.txt` in your project directory.
  
* **`crypto-heuristic-analyzer`** Analyzes the Code Section and keeps track of multiple factors and ratios such as the presence of certain bitwise operations, computation operations, and memory operations to determine a heuristically deduced "score" for WASM funcs. The scores of the WASM funcs are then ranked and dumped to `crypto-heuristic-rankings.txt` in your project directory. A higher score indicates a higher likelihood the func utilizes crypto.

* **`store-ops-data`**: Analyzes the Code Section and locates any instances of memory store instructions made in funcs. It'll output data on the address, value, and type of store made. In addition, it will also provide data if the store instruction is an expression directly containing an XOR operation (note that, at the moment, if the expression is referred to rather than being inlined, it will not recognize the use of the XOR even if logically the referrer evaluates to an expression with one). Data is dumped to `store-ops-data.txt` in your project directory.

* **`struct-solver`**: Analyzes the Code Section and locates instances of memory store instructions. However, instead of just outputting the data like `store-ops-data does`, it will attempt to identify structs by identifying stores made at multiple different offsets consecutively. Note that this can also identify structures like arrays/vectors too if it's all the same type. Data is dumped to `potential-structs.txt` in your project directory.

* **`xor-stores`**: Analyzes the Code Section and locates any instances of *direct* xor memory stores. This means that only memory stores that come immediately after an xor instruction are dumped. It is essentially a more powerful and direct analysis of XOR in memory stores compared to the feature store-ops-data has for XOR detection, which is a bit looser. A check for a direct xor store is often a key telltale sign of where encryption/decryption occurs, as many algorithms end with a xor encryption cipher. Data is dumped to `xor-stores.txt` in your project directory.

* **`func-lens`**: Simply tracks and sorts (via binary search insertions) all wasm func lengths. Many WebAssembly compilers will often compile much of the important code for clients (for example the main game loop for .io games), into extremely large funcs. Thus, having a relative ranking and output of func lengths can be useful. Data is dumped to `func-len-rankings.txt` in your project directory.
---
