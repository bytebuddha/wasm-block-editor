import init, { run_app } from './pkg/editor.js';

async function main() {
   await init('./pkg/editor_bg.wasm');
   run_app();
}
main()