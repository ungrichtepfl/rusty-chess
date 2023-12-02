import { Universe } from "rusty-chess-wasm";

const pre = document.getElementById("rusty-chess-wasm-canvas");
const width = 64;
const height = 32;

const universe = Universe.new(width, height);

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function renderLoop() {
  pre.textContent = universe.render();
  universe.tick();

  sleep(100).then(() => requestAnimationFrame(renderLoop));
}

requestAnimationFrame(renderLoop);
