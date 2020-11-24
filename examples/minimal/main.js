import init, { run_app } from './pkg/minimal.js';

async function main() {
   await init('./pkg/minimal_bg.wasm');
   run_app();
}
main()