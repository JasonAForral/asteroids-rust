use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::TAU;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    player: Player,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
    score: u32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Game {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let player = Player::new(
            canvas.width() as f64 / 2.0,
            canvas.height() as f64 / 2.0,
        );

        let mut asteroids = Vec::new();
        for _ in 0..5 {
            asteroids.push(Asteroid::new(
                rand::random::<f64>() * canvas.width() as f64,
                rand::random::<f64>() * canvas.height() as f64,
            ));
        }

        Game {
            canvas,
            context,
            player,
            asteroids,
            bullets: Vec::new(),
            score: 0,
        }
    }

    pub fn update(&mut self) {
        let (width, height) = (self.canvas.width() as f64, self.canvas.height() as f64);
        self.player.update((width, height));
        
        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }
        
        // Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update((width, height));
        }

        // Remove bullets that are off screen
        self.bullets.retain(|bullet| {
            bullet.x >= 0.0
                && bullet.x <= self.canvas.width() as f64
                && bullet.y >= 0.0
                && bullet.y <= self.canvas.height() as f64
        });

        // Check collisions
        self.check_collisions();
    }

    pub fn render(&self) {
        // Clear canvas
        self.context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        
        // Draw player
        self.player.draw(&self.context);
        
        // Draw asteroids
        for asteroid in &self.asteroids {
            asteroid.draw(&self.context);
        }
        
        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw(&self.context);
        }
    }

    pub fn shoot(&mut self) {
        self.bullets.push(self.player.shoot());
    }

    pub fn rotate_left(&mut self) {
        self.player.rotate(-0.1);
    }

    pub fn rotate_right(&mut self) {
        self.player.rotate(0.1);
    }

    pub fn thrust(&mut self) {
        self.player.thrust();
    }

    fn check_collisions(&mut self) {
        // Check bullet-asteroid collisions
        let mut i = 0;
        while i < self.bullets.len() {
            let mut j = 0;
            while j < self.asteroids.len() {
                if self.bullets[i].collides_with(&self.asteroids[j]) {
                    self.bullets.remove(i);
                    self.asteroids.remove(j);
                    self.score += 100;
                    break;
                }
                j += 1;
            }
            if i < self.bullets.len() {
                i += 1;
            }
        }
    }
}

struct Player {
    x: f64,
    y: f64,
    angle: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Player {
    fn new(x: f64, y: f64) -> Player {
        Player {
            x,
            y,
            angle: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    fn update(&mut self, (width, height): (f64, f64)) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        let size = 20.0;

        if self.x > width + size { self.x -= width + size + size }
        if self.x < 0.0 - size { self.x += width + size + size }

        if self.y > height + size { self.y -= height + size + size }
        if self.y < 0.0 - size { self.y += height  + size + size }
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.save();
        context.translate(self.x, self.y).unwrap();
        context.rotate(self.angle).unwrap();
        
        context.begin_path();
        context.move_to(0.0, -20.0);
        context.line_to(10.0, 10.0);
        context.line_to(-10.0, 10.0);
        context.close_path();
        
        context.set_stroke_style_str("white");
        context.stroke();
        
        context.restore();
    }

    fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }

    fn thrust(&mut self) {
        let (sin, cos) = self.angle.sin_cos();
        self.velocity_x += sin * 0.5;
        self.velocity_y -= cos * 0.5;
    }

    fn shoot(&self) -> Bullet {
        let (sin, cos) = self.angle.sin_cos();
        Bullet {
            x: self.x + sin * 20.0,
            y: self.y - cos * 20.0,
            velocity_x: sin * 10.0 + self.velocity_x,
            velocity_y: -cos * 10.0 + self.velocity_y,
        }
    }
}

struct Asteroid {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    size: f64,
}

impl Asteroid {
    fn new(x: f64, y: f64) -> Asteroid {
        Asteroid {
            x,
            y,
            velocity_x: (rand::random::<f64>() - 0.5) * 2.0,
            velocity_y: (rand::random::<f64>() - 0.5) * 2.0,
            size: 20.0,
        }
    }

    fn update(&mut self, (width, height): (f64, f64)) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        let size = self.size;

        if self.x > width + size { self.x -= width + size + size }
        if self.x < 0.0 - size { self.x += width + size + size }

        if self.y > height + size { self.y -= height + size + size }
        if self.y < 0.0 - size { self.y += height  + size + size }
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(self.x, self.y, self.size, 0.0, TAU).unwrap();
        context.set_stroke_style_str("white");
        context.stroke();
    }
}

struct Bullet {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Bullet {
    fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(self.x, self.y, 2.0, 0.0, TAU).unwrap();
        context.set_fill_style_str("white");
        context.fill();
    }

    fn collides_with(&self, asteroid: &Asteroid) -> bool {
        let dx = self.x - asteroid.x;
        let dy = self.y - asteroid.y;
        let distance_sq = dx * dx + dy * dy;
        distance_sq < asteroid.size * asteroid.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(100.0, 100.0);
        assert_eq!(player.x, 100.0);
        assert_eq!(player.y, 100.0);
        assert_eq!(player.angle, 0.0);
        assert_eq!(player.velocity_x, 0.0);
        assert_eq!(player.velocity_y, 0.0);
    }

    #[test]
    fn test_player_rotation() {
        let mut player = Player::new(100.0, 100.0);
        player.rotate(0.5);
        assert!((player.angle - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_player_thrust() {
        let mut player = Player::new(100.0, 100.0);
        player.thrust();
        assert!(player.velocity_x != 0.0 || player.velocity_y != 0.0);
    }

    #[test]
    fn test_player_movement() {
        let mut player = Player::new(100.0, 100.0);
        let initial_x = player.x;
        let initial_y = player.y;
        player.velocity_x = 1.0;
        player.velocity_y = 1.0;
        player.update((500.0, 500.0));
        assert_eq!(player.x, initial_x + 1.0);
        assert_eq!(player.y, initial_y + 1.0);
    }

    #[test]
    fn test_asteroid_creation() {
        let asteroid = Asteroid::new(100.0, 100.0);
        assert_eq!(asteroid.x, 100.0);
        assert_eq!(asteroid.y, 100.0);
        assert_eq!(asteroid.size, 20.0);
        assert!(asteroid.velocity_x.abs() <= 1.0);
        assert!(asteroid.velocity_y.abs() <= 1.0);
    }

    #[test]
    fn test_asteroid_movement() {
        let mut asteroid = Asteroid::new(100.0, 100.0);
        let initial_x = asteroid.x;
        let initial_y = asteroid.y;
        asteroid.update((500.0, 500.0));
        assert!(asteroid.x != initial_x || asteroid.y != initial_y);
    }

    #[test]
    fn test_bullet_creation() {
        let player = Player::new(100.0, 100.0);
        let bullet = player.shoot();
        assert!(bullet.x > 0.0);
        assert!(bullet.y > 0.0);
        assert!(bullet.velocity_x != 0.0 || bullet.velocity_y != 0.0);
    }

    #[test]
    fn test_bullet_movement() {
        let mut bullet = Bullet {
            x: 100.0,
            y: 100.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
        };
        let initial_x = bullet.x;
        let initial_y = bullet.y;
        bullet.update();
        assert_eq!(bullet.x, initial_x + 1.0);
        assert_eq!(bullet.y, initial_y + 1.0);
    }

    #[test]
    fn test_collision_detection() {
        let bullet = Bullet {
            x: 100.0,
            y: 100.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        };
        let asteroid = Asteroid {
            x: 100.0,
            y: 100.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            size: 20.0,
        };
        assert!(bullet.collides_with(&asteroid));

        let bullet = Bullet {
            x: 150.0,
            y: 150.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        };
        assert!(!bullet.collides_with(&asteroid));
    }

    #[test]
    fn test_game_mechanics() {
        // Create a mock game state with just the core game logic
        let mut player = Player::new(400.0, 300.0);
        let mut asteroids = vec![
            Asteroid::new(100.0, 100.0),
            Asteroid::new(200.0, 200.0),
            Asteroid::new(300.0, 300.0),
        ];
        let mut bullets = Vec::new();
        let mut score = 0;
        let canvas_dimensions = (500.0, 500.0);

        // Test shooting
        let initial_bullet_count = bullets.len();
        bullets.push(player.shoot());
        assert_eq!(bullets.len(), initial_bullet_count + 1);

        // Test rotation
        let initial_angle = player.angle;
        player.rotate(-0.1); // rotate left
        assert!(player.angle < initial_angle);
        player.rotate(0.1); // rotate right
        assert!((player.angle - initial_angle).abs() < 0.0001); // Should be back to initial angle

        // Test thrust
        let initial_velocity_x = player.velocity_x;
        let initial_velocity_y = player.velocity_y;
        player.thrust();
        assert!(player.velocity_x != initial_velocity_x || 
                player.velocity_y != initial_velocity_y);

        // Test movement
        let initial_x = player.x;
        let initial_y = player.y;
        player.update(canvas_dimensions);
        assert!(player.x != initial_x || player.y != initial_y);

        // Test asteroid movement
        for asteroid in &mut asteroids {
            let initial_x = asteroid.x;
            let initial_y = asteroid.y;
            asteroid.update(canvas_dimensions);
            assert!(asteroid.x != initial_x || asteroid.y != initial_y);
        }

        // Test bullet movement
        for bullet in &mut bullets {
            let initial_x = bullet.x;
            let initial_y = bullet.y;
            bullet.update();
            assert!(bullet.x != initial_x || bullet.y != initial_y);
        }

        // Test collision detection
        let bullet = Bullet {
            x: 100.0,
            y: 100.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        };
        let asteroid = &asteroids[0];
        assert!(bullet.collides_with(asteroid));
    }
}
