import { ChessGame, Piece } from "rusty-chess-wasm";
import { memory } from "rusty-chess-wasm/rusty_chess_wasm_bg.wasm";

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const CELL_SIZE = 80; // px
const GRID_COLOR = "#000000";
const BLACK_CELL_COLOR = "#999999";
const WHITE_CELL_COLOR = "#FFFFFF";
const FPS = 2;
const CHESS_BORD_COLS = 8;
const CHESS_BOARD_ROWS = 8;

const bishop_b = new Image();
bishop_b.src = "bishop-b.svg";
const bishop_w = new Image();
bishop_w.src = "bishop-w.svg";
const king_b = new Image();
king_b.src = "king-b.svg";
const king_w = new Image();
king_w.src = "king-w.svg";
const knight_b = new Image();
knight_b.src = "knight-b.svg";
const knight_w = new Image();
knight_w.src = "knight-w.svg";
const pawn_b = new Image();
pawn_b.src = "pawn-b.svg";
const pawn_w = new Image();
pawn_w.src = "pawn-w.svg";
const queen_b = new Image();
queen_b.src = "queen-b.svg";
const queen_w = new Image();
queen_w.src = "queen-w.svg";
const rook_b = new Image();
rook_b.src = "rook-b.svg";
const rook_w = new Image();
rook_w.src = "rook-w.svg";

const canvas = document.getElementById("rusty-chess-wasm-canvas");
let chessGame = ChessGame.new();
let boardPtr = chessGame.get_game_board();
let board = new Uint8Array(memory.buffer, boardPtr, 64);
function print_board() {
  let board_str = "";
  for (let i = 0; i < 8; i++) {
    for (let j = 0; j < 8; j++) {
      const idx = getIndex(i, j);
      board_str += board[idx] + " ";
    }
    board_str += "\n";
  }
  console.log(board_str);
}

const cell_space = CELL_SIZE;

canvas.height = cell_space * CHESS_BOARD_ROWS;
canvas.width = cell_space * CHESS_BORD_COLS;

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
  ctx.lineWidth = 5;

  ctx.moveTo(0, 0);
  ctx.lineTo(0, canvas.height);
  ctx.moveTo(canvas.width, 0);
  ctx.lineTo(canvas.width, canvas.height);

  ctx.moveTo(0, 0);
  ctx.lineTo(canvas.width, 0);
  ctx.moveTo(0, canvas.height);
  ctx.lineTo(canvas.width, canvas.height);

  ctx.fillStyle = BLACK_CELL_COLOR;
  for (let i = 0; i < CHESS_BOARD_ROWS; i++) {
    const start_idx = i % 2 ? 1 : 0;
    for (let j = start_idx; j < CHESS_BORD_COLS; j += 2) {
      ctx.fillRect(j * CELL_SIZE, i * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    }
  }
  ctx.fillStyle = WHITE_CELL_COLOR;
  for (let i = 0; i < CHESS_BOARD_ROWS; i++) {
    const start_idx = i % 2 ? 0 : 1;
    for (let j = start_idx; j < CHESS_BORD_COLS; j += 2) {
      ctx.fillRect(j * CELL_SIZE, i * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    }
  }
  ctx.stroke();
}

function getIndex(row, column) {
  row = 7 - row;
  return column * 8 + row;
}

function drawBoard() {
  print_board();
  for (let row = 0; row < CHESS_BOARD_ROWS; row++) {
    for (let col = 0; col < CHESS_BORD_COLS; col++) {
      const idx = getIndex(row, col);
      const piece = board[idx];
      let image = null;
      switch (piece) {
        case Piece.PawnBlack:
          image = pawn_b;
          break;
        case Piece.PawnWhite:
          image = pawn_w;
          break;
        case Piece.RookBlack:
          image = rook_b;
          break;
        case Piece.RookWhite:
          image = rook_w;
          break;
        case Piece.KnightBlack:
          image = knight_b;
          break;
        case Piece.KnightWhite:
          image = knight_w;
          break;
        case Piece.BishopBlack:
          image = bishop_b;
          break;
        case Piece.BishopWhite:
          image = bishop_w;
          break;
        case Piece.QueenBlack:
          image = queen_b;
          break;
        case Piece.QueenWhite:
          image = queen_w;
          break;
        case Piece.KingBlack:
          image = king_b;
          break;
        case Piece.KingWhite:
          image = king_w;
          break;
        default:
          break;
      }
      image &&
        ctx.drawImage(
          image,
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );
    }
  }
}

canvas.addEventListener("click", (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / CELL_SIZE), CHESS_BOARD_ROWS - 1);
  const col = Math.min(Math.floor(canvasLeft / CELL_SIZE), CHESS_BORD_COLS - 1);
  renderLoop(-Infinity);
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
  renderLoop(-Infinity);
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
  boardPtr = chessGame.get_game_board();
  board = new Uint8Array(memory.buffer, boardPtr, 64);
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
let previous_timestamp = -Infinity;
function renderLoop() {
  fps.render();

  // pre.textContent = chessGame.render();
  const userOutput = chessGame.play_randomly_aggressive();

  boardPtr = chessGame.get_game_board();
  board = new Uint8Array(memory.buffer, boardPtr, 64);
  drawGrid();
  drawBoard();

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
    const position = userOutput?.promotion_pos();
    console.log("Promotion: ", position);
    finish();
    return;
  } else if (userOutput?.is_draw()) {
    console.log("Draw");
    finish();
    return;
  }

  sleep((1000 * 1) / FPS).then(() => {
    if (!isPaused()) {
      renderLoop();
    }
  });
}

drawGrid();
drawBoard();
play();
