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

The game includes a comprehensive test suite that can be run using `cargo test`. These tests cover all the core game mechanics:

```bash
cargo test
```

The test suite includes:

### Player Tests
- Player creation and initialization
- Rotation mechanics
- Thrust mechanics
- Movement and position updates

### Asteroid Tests
- Asteroid creation and initialization
- Movement mechanics
- Size verification

### Bullet Tests
- Bullet creation and initialization
- Movement mechanics
- Collision detection

### Game Mechanics Tests
- Combined tests of player, asteroid, and bullet interactions
- Movement verification
- Collision detection
- Score tracking

All tests are designed to run in a pure Rust environment without any Web API dependencies, making them fast and reliable for development and CI/CD pipelines. 
