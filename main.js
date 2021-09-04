import init, { run_app } from "./pkg/bertrand.js";
async function main() {
  await init("/pkg/bertrand_bg.wasm");
  run_app();
}
main();
