
pub mod entities{
    
    const SCREEN_HEIGHT: i32 = 50;
    
    use bracket_lib::prelude::{ to_cp437, BLACK, RED, YELLOW, BTerm, RandomNumberGenerator };

    pub struct Player {
        x: i32,
        y: i32,
        velocity: f32,
    }

    pub struct Obstacle {
        x: i32,
        gap_y: i32,
        size: i32,
    }
    
    impl Obstacle {

        pub fn size(&self) -> i32 {
            self.size
        }


        pub fn y_gap(&self) -> i32 {
            self.gap_y
        }

        pub fn x_pos(&self) -> i32 {
            self.x
        }

        pub fn new(x: i32, score: i32) -> Self {
            let mut random = RandomNumberGenerator::new();
            Obstacle {
                x, 
                gap_y: random.range(10,40),
                size: i32::max(2,20-score)
            }
        }
    
        pub fn hit_obstacle(&self, player: &Player) -> bool {
            
            let half_size = self.size() / 2;
            let does_x_match = player.x_pos() == self.x_pos();
    
            // within gap bounds
            let player_above_gap = player.y_pos() < self.y_gap() - half_size;
            let player_below_gap  = player.y_pos() > self.y_gap() + half_size;
    
            does_x_match && ( player_above_gap || player_below_gap)
        }
    
        pub fn render(&mut self, ctx: &mut BTerm, player_x: i32){
    
            let screen_x = self.x - player_x;
            let half_size = self.size / 2;
    
    
            for y in 0..self.gap_y - half_size {
                ctx.set(
                    screen_x,
                    y, 
                    RED,
                    BLACK,
                    to_cp437('|'),
                );
            }
    
            for y in self.gap_y + half_size..SCREEN_HEIGHT {
                ctx.set(
                    screen_x,
                    y,
                    RED,
                    BLACK,
                    to_cp437('|'),
                );
            }
    
        }
    }


    impl Player {

        pub fn y_pos(&self) -> i32 {
            self.y
        }

        pub fn x_pos(&self) -> i32 {
            self.x
        }

        pub fn new(x:i32, y: i32) -> Self {
            Player{
                x,
                y,
                velocity: 0.0,
            }
        }
    
        pub fn render(&mut self, ctx: &mut BTerm){
            ctx.set(
                0,
                self.y,
                YELLOW,
                BLACK,
                to_cp437('@')
            );
        }
    
        pub fn gravity_and_move(&mut self){
            if self.velocity < 2.0 {
                self.velocity += 0.2;
            }
    
            self.y += self.velocity as i32;
    
            self.x += 1;
    
            if self.y < 0 {
                self.y = 0;
            }
        }
    
        pub fn flap(&mut self){
            self.velocity = -2.0;
        }
    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entities::{Player, Obstacle};

    #[test]
    fn test_new_player() {
        let subject  = Player::new(0, 0);
        let (expected_x, expected_y) = (0, 0);

        assert_eq!(subject.x_pos(), expected_x);
        assert_eq!(subject.y_pos(), expected_y);
    }
}