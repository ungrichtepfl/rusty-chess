import { Universe, Cell } from "rusty-chess-wasm";
import { memory } from "rusty-chess-wasm/rusty_chess_wasm_bg.wasm";

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.getElementById("rusty-chess-wasm-canvas");
const cell_number_col = 64;
const cell_number_row = 64;
const universe = Universe.new(cell_number_col, cell_number_row);

const padding = 1;
const cell_space = CELL_SIZE + padding;

canvas.height = cell_space * cell_number_row + padding;
canvas.width = cell_space * cell_number_col + padding;

const ctx = canvas.getContext("2d");

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

function drawCells() {
  const cellPtr = universe.cells();
  const cells = new Uint8Array(
    memory.buffer,
    cellPtr,
    cell_number_row * cell_number_col
  );

  ctx.beginPath();

  for (let row = 0; row < cell_number_row; row++) {
    for (let col = 0; col < cell_number_col; col++) {
      const idx = getIndex(row, col);
      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
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
  canvas.textContent = universe.render();
  universe.tick();

  drawGrid();
  drawCells();
  // animationId = requestAnimationFrame(renderLoop);
  sleep(100).then(() => {
    if (!isPaused()) {
      animationId = requestAnimationFrame(renderLoop);
    }
  });
}

play();
