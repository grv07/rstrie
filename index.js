// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "../pkg/rstrie.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const wasm = await init("./pkg/rstrie_bg.wasm");

  // Call the Add function export from wasm, save the result
  const addResult = wasm.add(24, 24);
  const n = wasm.node_new();
  const tree = wasm.trie_new(n);
  tree.trie_insert(tree, "data", 3);

  console.log(tree);

  // Set the result onto the body
  document.body.textContent = `Hello World! addResult: ${addResult}`;
};
runWasm();
