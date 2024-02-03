use macroquad::prelude::*;

#[derive(Debug)]
pub struct Timer {
    duration: f32,
    started: bool,
    pub time_left: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            time_left: duration,
            started: false,
            duration: duration,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.started {
            return;
        };
        self.time_left = (self.time_left - dt).max(0.0);
        if self.is_done() {
            self.started = false
        };
    }

    pub fn is_done(&mut self) -> bool {
        return self.time_left == 0.0;
    }

    pub fn is_started(&mut self) -> bool {
        return self.started;
    }

    pub fn reset(&mut self) {
        self.time_left = self.duration
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.started = false;
    }
}

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub speed: f32,
}

#[derive(Debug)]
pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub speed: f32,
    pub timer: Timer,
}

fn lerp(a: f32, b: f32, f: f32) -> f32 {
    return a + f * (b - a);
}

fn spawn_enemies(enemies: &mut Vec<Enemy>, player: &Player) {
    let enemy_position = Vec2 {
        x: rand::gen_range(30.0, screen_width() - 30.0),
        y: -15.0,
    };
    let enemy_velocity = (player.position - enemy_position).normalize() * 500.0;

    let new_enemy = Enemy {
        position: enemy_position,
        velocity: enemy_velocity,
        radius: 15.0,
        speed: 0.0,
        timer: Timer::new(0.3),
    };
    enemies.push(new_enemy);
}

fn is_enemy_killed(enemy: &Enemy, player: &Player) -> bool {
    return player.position.y < enemy.position.y
        && (((player.position - player.velocity).x <= enemy.position.x
            && (player.position + player.velocity).x >= enemy.position.x)
            || ((player.position - player.velocity).x >= enemy.position.x
                && (player.position + player.velocity).x <= enemy.position.x));
}

fn handle_player_movement(player: &mut Player, delta: f32) {
    player.velocity.y += 80.0 * delta;

    if player.position.y >= (screen_height() - player.radius) {
        player.position.y = screen_height() - player.radius;
        player.velocity.y = 0.0;
        if is_key_pressed(KeyCode::Up) {
            player.velocity.y = -25.0;
        }
    }
    let mut direction: f32 = 0.0;
    if is_key_down(KeyCode::Right) {
        direction += 1.0;
    }
    if is_key_down(KeyCode::Left) {
        direction -= 1.0;
    }
    draw_circle(player.position.x, player.position.y, player.radius, YELLOW);

    player.position += player.velocity;
    if direction.abs() > 0.0 {
        player.velocity.x = lerp(player.velocity.x, direction * player.speed, 10.0 * delta);
    } else {
        player.velocity.x = lerp(player.velocity.x, direction * player.speed, 20.0 * delta);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut player = Player {
        position: Vec2 {
            x: screen_width() * 0.5,
            y: 30.0,
        },
        velocity: Vec2::ZERO,
        radius: 10.0,
        speed: 15.0,
    };
    let mut enemies = Vec::<Enemy>::new();
    let mut enemy_timer = Timer::new(0.5);
    enemy_timer.start();
    println!("Enemies: {:?}", enemies);
    loop {
        let delta = get_frame_time();

        clear_background(BLACK);
        handle_player_movement(&mut player, delta);

        enemy_timer.update(delta);
        if enemy_timer.is_done() {
            spawn_enemies(&mut enemies, &player);
            enemy_timer.reset();
            enemy_timer.start();
        }

        enemies.retain_mut(|enemy| {
            enemy.timer.update(delta);
            enemy.position += enemy.velocity * delta;

            if enemy.position.x + enemy.radius > screen_width()
                || enemy.position.x - enemy.radius < 0.0
            {
                enemy.velocity.x *= -1.0;
            }

            if enemy.position.y + enemy.radius > screen_height()
                || enemy.position.y - enemy.radius < -30.0
            {
                enemy.velocity.y *= -1.0;
            }
            if is_enemy_killed(&enemy, &player) {
                enemy.timer.start();
            }
            if enemy.timer.is_done() {
                return false;
            }
            if enemy.position.distance_squared(player.position)
                < (enemy.radius + player.radius).powi(2)
            {
                panic!("DEATH");
            }
            if enemy.timer.is_started() {
                draw_circle(enemy.position.x, enemy.position.y, enemy.radius, GREEN);
            } else {
                draw_circle(enemy.position.x, enemy.position.y, enemy.radius, RED);
            }
            return true;
        });

        next_frame().await
    }
}
