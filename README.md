# Tic-Tac-Toe

A client-side Tic-Tac-Toe game built with **Rust + Leptos 0.8** compiled to WASM, styled with **TailwindCSS 3**.

## Features

- **Two game modes**: 2-Player local and Vs AI
- **AI with 3 difficulty levels**: Easy (random), Medium (70% optimal), Hard (minimax)
- **Undo/Redo**: Full move history; in VsAI mode, undoes/redoes full turns (player + AI)
- **Dark mode**: Toggle persisted to `localStorage`
- **Score persistence**: Wins/draws saved across sessions
- **Winning line highlight**: Winning cells flash green

## Quick Start

```bash
# Install tools (if needed)
cargo install trunk
rustup target add wasm32-unknown-unknown

# Install Tailwind deps
pnpm install

# Start dev server
trunk serve

# Build for production (outputs to dist/)
trunk build
```

## Project Structure

```
src/
  main.rs     — Entry point, mounts <App/> to body
  app.rs      — Root component, state management, event handlers
  game.rs     — Game logic: Board, GameState, AI (minimax)
  ui/
    mod.rs    — Re-exports Board, Controls, Score
    board.rs  — 3x3 grid component
    cell.rs   — Individual cell button
    score.rs  — Score display + turn indicator
    controls.rs — Mode/difficulty buttons, undo/redo, reset
```

## Tech Stack

| Tool | Purpose |
|---|---|
| Rust 2024 | Language |
| Leptos 0.8 (CSR) | Reactive UI framework |
| Trunk | WASM bundler |
| TailwindCSS 3 | Styling |

## Commands

```bash
trunk serve    # Development server on port 8080
trunk build    # Production build to dist/
cargo test     # Run unit tests (history, undo/redo, AI)
```

## Dependencies

See `Cargo.toml`. Key crates: `leptos` (CSR mode), `serde`/`serde_json` for `localStorage` persistence, `wasm-bindgen`/`web-sys` for browser API access, `fastrand` for AI randomness.
