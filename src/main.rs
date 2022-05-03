use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut args = std::env::args(); args.next();

    let mut size = (args.next().unwrap_or_else(|| "7".to_string())).parse().unwrap();
    let p1 = args.next().unwrap_or("player 1".to_string());
    let p2 = args.next().unwrap_or("player 2".to_string());

    size += 2;
    let mut grid = vec![vec![0u8; size]; size];
    grid[size/2-3][0] = 3; //player 1
    grid[size/2-1][0] = 5; //player 1 score
    grid[size/2][0] = 6;   //colon inbetween
    grid[size/2+1][0] = 7; //player 2 score
    grid[size/2+2][0] = 4; //player 2

    let mut scores = [0usize; 2];

    let mut current_player = 1u8;

    let mut victory_screen_time = 0;

    loop {
        let w = screen_width();
        let h = screen_height();
        if w != h {
            request_new_screen_size(w, w); //resize to a square to avoid fucky behavior
        } else {
            clear_background(WHITE);

            if victory_screen_time > 0 {
                victory_screen_time-=1;
                draw_text(&format!("PLAYER #{} WON", (scores[0] < scores[1]) as u8 + 1), (w/2.)*0.6, (h/2.)*0.9, w/15., BLACK);
            } else {
                /* #region DRAW GRID */
                let f32_step = w/(size as f32);
                let step = f32_step as usize;
                for i in 1..size {
                    let x = f32_step * i as f32;
                    draw_line(x, f32_step, x, h-f32_step, 2., BLACK);
                    draw_line(f32_step, x, w-f32_step, x, 2., BLACK);
                }
                /* #endregion */
    
                /* #region CHECK INPUT */
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (x,y) = mouse_position();
    
                    //draw_circle(x, y, 20., RED);
                    if x > f32_step && x < (step*(size-1)) as f32
                    && y > f32_step && y < (step*(size-1)) as f32 {
                        let tile = &mut grid[(x as usize)/step][(y as usize)/step];
                        
                        if *tile == 0 {
                            *tile = current_player; //1 == circle, 2 == cross
                            current_player ^= 0b0000_0011; //XOR to flip the last two bits, switches between 1 and 2
                        }
                    }
                }
                /* #endregion */
    
                /* #region DRAW TILES & CHECK FOR WINS */
                for (x, row) in grid.iter().enumerate() {
                    let mut successive_tiles = 0u8; //counter, first bit indicates the player 
                    for (y, tile) in row.iter().enumerate() {
                        match tile {
                            1 => { // == circle
                                if successive_tiles & 0b1000_0000 == 0 { //check the player
                                    successive_tiles += 1;
                                } else if successive_tiles == 0 {
                                    successive_tiles = 1;
                                } else {successive_tiles = 0;}
                                draw_circle(f32_step*(x as f32 + 0.5), f32_step*(y as f32 + 0.5), f32_step/3., BLACK)
                            }
                            2 => { // == cross
                                if successive_tiles & 0b1000_0000 == 0b1000_0000 { //check the player
                                    successive_tiles += 1;
                                } else if successive_tiles == 0 {
                                    successive_tiles = 0b1000_0001;
                                } else {successive_tiles = 0;}
                                let (x1, y1) = (f32_step*(x as f32 + 0.15), f32_step*(y as f32 + 0.15));
                                let (x2, y2) = (f32_step*(x as f32 + 0.85), f32_step*(y as f32 + 0.85));
                                draw_line(x1, y1, x2, y2, 5., BLACK);
                                draw_line(x1, y2, x2, y1, 5., BLACK);
                            }
                            3 => { // == player 1 name
                                draw_text(&p1, f32_step*(x as f32), f32_step*(y as f32 + 0.8), w/20., BLACK);
                            }
                            4 => { // == player 2 name
                                draw_text(&p2, f32_step*(x as f32), f32_step*(y as f32 + 0.8), w/20., BLACK);
                            }
                            5 => { // == player 1 score
                                draw_text(&scores[0].to_string(), f32_step*(x as f32 + 0.3), f32_step*(y as f32 + 0.8), w/10., BLACK);
                            }
                            6 => { // == colon between scores
                                draw_text(":", f32_step*(x as f32 + 0.3), f32_step*(y as f32 + 0.8), w/10., BLACK);
                            }
                            7 => { // == player 2 score
                                draw_text(&scores[1].to_string(), f32_step*(x as f32 + 0.3), f32_step*(y as f32 + 0.8), w/10., BLACK);
                            }
                            _ => () //0 == empty
                        }
                    }
                    if successive_tiles & 0b0111_1111 == 4 {
                        scores[(successive_tiles >> 7) as usize]+=1;
                        victory_screen_time = 180; //3s
                        
                        //reset grid
                        grid = vec![vec![0u8; size]; size];
                        grid[0][size/2] = 3;
                        grid[size-1][size/2] = 4;
                        grid[0][size/2+1] = 5;
                        grid[size-1][size/2+1] = 6;
                        break
                    }
                }
                /* #endregion */
            }
        }

        next_frame().await
    }
}