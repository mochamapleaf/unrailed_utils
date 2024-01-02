import init, * as wasm from '../pkg/unrailed_seed_analyzer.js';

async function run() {
  await init();
}

run();
window.wasm = wasm;