import init, { run_app } from './pkg/tiles.js';
async function main() {
   await init('/pkg/tiles_bg.wasm');
   run_app();
}
main()
