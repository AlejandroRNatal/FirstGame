use bracket_lib::prelude::*;

mod entities;
use crate::entities::entities::{Obstacle, Player};

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

impl State {
    fn new() -> Self {
        State{
            player: Player::new(5,25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            score:0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }

    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);

        ctx.print(0,0, "Press SPACE to flap");
        ctx.print(0,1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x_pos());

        if self.player.x_pos() > self.obstacle.x_pos() {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x_pos() + SCREEN_WIDTH, self.score
            );
        }

        if self.player.y_pos() > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }

    }

    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(6, &format!(" Score: {}", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}, //ignore all others
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}, //ignore all others
            }
        }
    }

    fn restart(&mut self){
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0 ;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm)
    {
        ctx.cls();
        ctx.print(1,1,"Hello, Bracket Terminal");

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon")
                                             .build()?;


    main_loop(context, State::new())
}
