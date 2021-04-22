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

use macroquad::prelude::Color;

pub const CANVAS_WIDTH: f32 = 320.0;
pub const CANVAS_HEIGHT: f32 = 200.0;

pub const FRAME_COLOR: Color = Color::new(0.0 / 255.0, 136.0 / 255.0, 255.0 / 255.0, 1.0);
pub const BACKGROUND_COLOR: Color = Color::new(0.0, 0.0, 170.0 / 255.0, 1.0);

pub mod canvas;

