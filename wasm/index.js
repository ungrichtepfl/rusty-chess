import { Universe, Cell } from "rusty-chess-wasm";
import { memory } from "rusty-chess-wasm/rusty_chess_wasm_bg.wasm";

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const FPS = 30;

const canvas = document.getElementById("rusty-chess-wasm-canvas");
const cell_number_col = 64;
const cell_number_row = 64;
const universe = Universe.new(cell_number_col, cell_number_row);


const padding = 1;
const cell_space = CELL_SIZE + padding;

canvas.height = cell_space * cell_number_row + padding;
canvas.width = cell_space * cell_number_col + padding;

const ctx = canvas.getContext("2d");

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= cell_number_col; i++) {
    ctx.moveTo(i * cell_space + padding, 0);
    ctx.lineTo(i * cell_space + padding, canvas.height);
  }

  for (let i = 0; i <= cell_number_row; i++) {
    ctx.moveTo(0, i * cell_space + padding);
    ctx.lineTo(canvas.width, i * cell_space + padding);
  }
  ctx.stroke();
}

function getIndex(row, column) {
  return row * cell_number_col + column;
}

const cellPtr = universe.cells();
const cells = new Uint8Array(
  memory.buffer,
  cellPtr,
  cell_number_row * cell_number_col
);

function drawCells() {

  ctx.beginPath();
  ctx.fillStyle = DEAD_COLOR; // only set once very slow
  for (let row = 0; row < cell_number_row; row++) {
    for (let col = 0; col < cell_number_col; col++) {

      const idx = getIndex(row, col);
      if (cells[idx] !== Cell.Dead) {
        continue;
      }

      ctx.fillRect(
        col * cell_space + padding,
        row * cell_space + padding,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.fillStyle = ALIVE_COLOR; // only set once very slow
  for (let row = 0; row < cell_number_row; row++) {
    for (let col = 0; col < cell_number_col; col++) {
      const idx = getIndex(row, col);
      if (cells[idx] !== Cell.Alive) {
        continue;
      }
      ctx.fillRect(
        col * cell_space + padding,
        row * cell_space + padding,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
}

canvas.addEventListener("click", (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(
    Math.floor(canvasTop / (CELL_SIZE + padding)),
    cell_number_row - 1
  );
  const col = Math.min(
    Math.floor(canvasLeft / (CELL_SIZE + padding)),
    cell_number_col - 1
  );

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

const playPauseButton = document.getElementById("play-pause");

let animationId = null;

let paused = false;

function isPaused() {
  return paused;
}

function play() {
  paused = false;
  playPauseButton.textContent = "⏸";
  renderLoop();
}
function pause() {
  paused = true;
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
}

playPauseButton.addEventListener("click", (event) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

function renderLoop() {

  fps.render();
  universe.tick();

  drawGrid();
  drawCells();
  // animationId = requestAnimationFrame(renderLoop);
  sleep(1000 * 1/FPS).then(() => {
    if (!isPaused()) {
      animationId = requestAnimationFrame(renderLoop);
    }
  });
}

play();
