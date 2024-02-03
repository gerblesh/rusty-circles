use macroquad::prelude::*;

pub struct Timer {
    duration: f32,
    pub time_left: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            time_left: duration,
            duration,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_left = (self.time_left - dt).max(0.0);
    }

    pub fn is_done(&mut self) -> bool {
        return self.time_left == 0.0;
    }

    pub fn reset(&mut self) {
        self.time_left = self.duration
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
}

fn lerp(a: f32, b: f32, f: f32) -> f32 {
    return a + f * (b - a);
}

fn spawn_enemies(enemies: &mut Vec<Enemy>, player: &Player) {
    let enemy_position = Vec2 {
        x: rand::gen_range(30.0, screen_width() - 30.0),
        y: 30.0,
    };
    let enemy_velocity = (player.position - enemy_position).normalize() * 500.0;

    let new_enemy = Enemy {
        position: enemy_position,
        velocity: enemy_velocity,
        radius: 15.0,
        speed: 0.0,
    };
    enemies.push(new_enemy);
}

fn handle_player_movement(player: &mut Player, delta: f32) {
    player.velocity.y += 1.0;

    if player.position.y >= (screen_height() - player.radius) {
        player.position.y = screen_height() - player.radius;
        player.velocity.y = 0.0;
        if is_key_pressed(KeyCode::Up) {
            player.velocity.y = -20.0;
        }
    }
    let mut direction: f32 = 0.0;
    if is_key_down(KeyCode::Right) {
        direction += 1.0;
    }
    if is_key_down(KeyCode::Left) {
        direction -= 1.0;
    }
    draw_circle(player.position.x, player.position.y, 15.0, YELLOW);

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
        radius: 15.0,
        speed: 20.0,
    };
    let mut enemies = Vec::<Enemy>::new();
    let mut enemy_timer = Timer::new(0.5);
    println!("Enemies: {:?}", enemies);
    loop {
        let delta = get_frame_time();

        clear_background(BLACK);
        handle_player_movement(&mut player, delta);

        enemy_timer.update(delta);
        if enemy_timer.is_done() {
            spawn_enemies(&mut enemies, &player);
            enemy_timer.reset();
        }
        let mut index = 0;
        while index < enemies.len() {
            let enemy = &mut enemies[index];

            enemy.position += enemy.velocity * delta;

            if enemy.position.x + enemy.radius > screen_width()
                || enemy.position.x - enemy.radius < 0.0
            {
                enemy.velocity = Vec2 {
                    x: -enemy.velocity.x,
                    y: enemy.velocity.y,
                };
            }

            if enemy.position.y + enemy.radius > screen_height()
                || enemy.position.y - enemy.radius < 0.0
            {
                enemy.velocity = Vec2 {
                    x: enemy.velocity.x,
                    y: -enemy.velocity.y,
                };
            }
            if player.position.y < enemy.position.y
                && (((player.position - player.velocity).x <= enemy.position.x
                    && (player.position + player.velocity).x >= enemy.position.x)
                    || ((player.position - player.velocity).x >= enemy.position.x
                        && (player.position + player.velocity).x <= enemy.position.x))
            {
                enemies.drain(index..index + 1);
            } else if enemy.position.distance_squared(player.position)
                < (enemy.radius + player.radius).powi(2)
            {
                // enemies.drain(index..index + 1);
                panic!("DEATH")
            } else {
                draw_circle(enemy.position.x, enemy.position.y, enemy.radius, RED);
                index += 1;
            }
        }

        next_frame().await
    }
}
