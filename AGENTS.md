# Tic-Tac-Toe Project

## Tech Stack
- **Rust 2024** edition
- **Leptos 0.8** (CSR mode - client-side rendering)
- **Trunk** - WASM bundler for serving
- **TailwindCSS 3** - styling via pnpm

## Commands

```bash
# Development
trunk serve

# Build (outputs to dist/)
trunk build

# Install Trunk (if needed)
cargo install trunk

# Add wasm target (required for Leptos)
rustup target add wasm32-unknown-unknown
```

## Key Files
- `src/main.rs` - Entry point, mount app to body
- `index.html` - HTML template with TailwindCSS link
- `Trunk.toml` - Build config (Tailwind version 3)
- `tailwind.config.js` - Tailwind configuration

## Dependencies Note
- `leptos_router` was removed - CSR mode doesn't use routing features
- Use `features = ["csr"]` for leptos client-side rendering

## Style with Tailwind
- Classes go directly in `view!` macros: `class="text-xl font-bold"`
- Input CSS at `style/input.css` with `@tailwind base; @tailwind components; @tailwind utilities;`