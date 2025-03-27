# Asteroids Game in Rust + WebAssembly

A classic Asteroids game implemented in Rust and compiled to WebAssembly.

## Prerequisites

- Rust (latest stable version)
- `wasm-pack` (install with `cargo install wasm-pack`)
- A web server (for serving the game)

## Building

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Serve the game using a local web server. You can use Python's built-in server:
```bash
python3 -m http.server 8000 --bind 127.0.0.1
```

3. Open your browser and navigate to `http://localhost:8000`

## Controls

- Left Arrow: Rotate left
- Right Arrow: Rotate right
- Up Arrow: Thrust
- Space: Shoot

## Game Features

- Player ship with rotation and thrust mechanics
- Destructible asteroids
- Bullet shooting mechanics
- Score tracking
- Collision detection
- Smooth animation using requestAnimationFrame

## Testing

The game includes a comprehensive test suite that can be run in both Node.js and browser environments.

### Running Tests in Browser

```bash
wasm-pack test --chrome
```

### Running Tests in Firefox

```bash
wasm-pack test --firefox
```

### Running Tests in Safari

```bash
wasm-pack test --safari
```

The test suite covers:
- Player creation and movement
- Asteroid creation and movement
- Bullet creation and movement
- Collision detection
- Game state management

Note: Browser tests require the appropriate browser to be installed on your system. 
