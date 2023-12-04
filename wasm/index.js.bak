import { ChessGame, UserOutputWrapper } from "rusty-chess-wasm";
import { memory } from "rusty-chess-wasm/rusty_chess_wasm_bg.wasm";

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const FPS = 3;
const CHESS_BORD_COLS = 8;
const CHESS_BOARD_ROWS = 8;

const canvas = document.getElementById("rusty-chess-wasm-canvas");
let chessGame = ChessGame.new();

const padding = 1;
const cell_space = CELL_SIZE + padding;

canvas.height = cell_space * CHESS_BOARD_ROWS + padding;
canvas.width = cell_space * CHESS_BORD_COLS + padding;

const ctx = canvas.getContext("2d");

const fps = new (class {
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
    const fps = (1 / delta) * 1000;

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
})();

function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= CHESS_BORD_COLS; i++) {
    ctx.moveTo(i * cell_space + padding, 0);
    ctx.lineTo(i * cell_space + padding, canvas.height);
  }

  for (let i = 0; i <= CHESS_BOARD_ROWS; i++) {
    ctx.moveTo(0, i * cell_space + padding);
    ctx.lineTo(canvas.width, i * cell_space + padding);
  }
  ctx.stroke();
}

function getIndex(row, column) {
  return row * CHESS_BORD_COLS + column;
}

function drawCells() {
  ctx.beginPath();
  for (let row = 0; row < CHESS_BOARD_ROWS; row++) {
    for (let col = 0; col < CHESS_BORD_COLS; col++) {
      // const idx = getIndex(row, col);
      // if (cells[idx] !== Cell.Dead) {
      //   continue;
      // }
      //
      // ctx.fillRect(
      //   col * cell_space + padding,
      //   row * cell_space + padding,
      //   CELL_SIZE,
      //   CELL_SIZE
      // );
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
    CHESS_BOARD_ROWS - 1
  );
  const col = Math.min(
    Math.floor(canvasLeft / (CELL_SIZE + padding)),
    CHESS_BORD_COLS - 1
  );

  // chessGame.toggle_cell(row, col);

  // drawGrid();
  // drawCells();
});

const playPauseButton = document.getElementById("play-pause");

let animationId = null;

let paused = false;
let finished = false;

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

function finish() {
  finished = true;
  pause();
}

function restart() {
  finished = false;
  chessGame = ChessGame.new();
  play();
}

playPauseButton.addEventListener("click", (event) => {
  if (finished) {
    restart();
  } else if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const pre = document.getElementById("rusty-chess-wasm-pre");

function renderLoop() {
  fps.render();

  pre.textContent = chessGame.render();
  const userOutput = chessGame.play_randomly_aggressive();

  if (userOutput?.is_check_mate()) {
    console.log("CheckMate");
    finish();
    return;
  } else if (userOutput?.is_stale_mate()) {
    console.log("StaleMate");
    finish();
    return;
  } else if (userOutput?.is_invalid_move()) {
    console.log("InvalidMove");
    finish();
    return;
  } else if (userOutput?.is_promotion()) {
    position = userOutput?.promotion_pos();
    console.log("Promotion: ", position);
    finish();
    return;
  } else if (userOutput?.is_draw()) {
    console.log("Draw");
    finish();
    return;
  }

  drawGrid();
  drawCells();

  // animationId = requestAnimationFrame(renderLoop);
  sleep((1000 * 1) / FPS).then(() => {
    if (!isPaused()) {
      animationId = requestAnimationFrame(renderLoop);
    }
  });
}

play();
