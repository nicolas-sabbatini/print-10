/*
  Copyright 2021 Nicolas Cesar Sabbatini Vrech

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use macroquad::prelude::*;
use macroquad::rand::{rand, srand};
use print_10::{CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND_COLOR, FRAME_COLOR};
use print_10::canvas::Canvas2D;

// Screen configuration
fn window_conf() -> Conf {
  Conf {
    window_title: "10 PRINT".to_owned(),
    window_width: (CANVAS_WIDTH as i32 + 16) * 3,
    window_height: (CANVAS_HEIGHT as i32 + 16) * 3,
    ..Default::default()
  }
}

enum Charter {
  Left,
  Right,
}

pub fn vec_idx_to_canvas(index: usize) -> (f32, f32) {
  let x: usize = index % 40;
  (x as f32, ((index - x) / 40) as f32)
}

#[macroquad::main(window_conf)]
async fn main() {
  // Create texture to draw
  let canvas = Canvas2D::new(CANVAS_WIDTH, CANVAS_HEIGHT);
  // Clear canvas
  canvas.draw_inside_canvas(&|| {
    clear_background(BACKGROUND_COLOR);
  });

  let mut vec_char: Vec<Charter> = Vec::new();

  // Seed random generator
  // Get time for the start of the app and multiply for a big number
  srand((get_time() * 1099511627776.0) as u64);

  loop {

    // Clear screen
    clear_background(FRAME_COLOR);

    // Push a new random charter
    if rand() < u32::MAX / 2 {
      vec_char.push(Charter::Left);
    } else {
      vec_char.push(Charter::Right);
    }

    if vec_char.len() > ((CANVAS_HEIGHT / 8.0) * (CANVAS_WIDTH / 8.0)) as usize {
      vec_char.drain(0..40);
    }

    canvas.draw_inside_canvas(&|| {
      clear_background(BACKGROUND_COLOR);
      for (i, char) in vec_char.iter().enumerate() {
        let (x, y) = vec_idx_to_canvas(i);
        match char {
          Charter::Left => {
            draw_line(x * 8.0 + 1.0, y * 8.0 + 1.0, x * 8.0 + 7.0, y * 8.0 + 7.0, 4.0, FRAME_COLOR);
            draw_triangle(Vec2::new(x * 8.0, y * 8.0), Vec2::new(x * 8.0 + 4.0, y * 8.0), Vec2::new(x * 8.0, y * 8.0 + 4.0), FRAME_COLOR);
            draw_triangle(Vec2::new(x * 8.0 + 8.0, y * 8.0 + 8.0), Vec2::new(x * 8.0 + 5.0, y * 8.0 + 8.0), Vec2::new(x * 8.0 + 8.0, y * 8.0 + 5.0), FRAME_COLOR);
          }
          Charter::Right => {
            draw_line(x * 8.0 + 1.0, y * 8.0 + 7.0, x * 8.0 + 7.0, y * 8.0 + 1.0, 4.0, FRAME_COLOR);
            draw_triangle(Vec2::new(x * 8.0, y * 8.0 + 8.0), Vec2::new(x * 8.0 + 3.0, y * 8.0 + 8.0), Vec2::new(x * 8.0, y * 8.0 + 4.0), FRAME_COLOR);
            draw_triangle(Vec2::new(x * 8.0 + 8.0, y * 8.0), Vec2::new(x * 8.0 + 4.0, y * 8.0), Vec2::new(x * 8.0 + 8.0, y * 8.0 + 3.0), FRAME_COLOR);
          }
        }
      }
    });

    // Calculate canvas dimensions and padding
    let (left_padding, top_padding, dimensions) = canvas
      .calculate_size_and_padding(screen_width() - 48.0, screen_height() - 48.0);

    // Draw canvas on screen
    draw_texture_ex(
      *canvas.get_texture(),
      left_padding + 24.0,
      top_padding + 24.0,
      WHITE,
      DrawTextureParams {
        // https://github.com/not-fl3/macroquad/issues/171
        flip_y: true,
        dest_size: Some(dimensions),
        ..Default::default()
      },
    );

    next_frame().await
  }
}
