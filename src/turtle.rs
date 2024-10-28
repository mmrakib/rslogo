use unsvg::{Image, COLORS, get_end_coordinates};

use crate::error::print_error;

use std::fmt;

pub struct Turtle {
    x: f64,
    y: f64,
    heading: f64,
    pen_down: bool,
    pen_color: i32,
    image: unsvg::Image,
    filename: String,
}

impl fmt::Debug for Turtle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Turtle")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("heading", &self.heading)
            .field("pen_down", &self.pen_down)
            .field("pen_color", &self.pen_color)
            .field("filename", &self.filename)
            .finish()
    }
}

impl Turtle {
    pub fn new(width: u32, height: u32, filename: String) -> Self {
        Turtle {
            x: (width / 2) as f64,
            y: (height / 2) as f64,
            heading: 0.0,
            pen_down: false,
            pen_color: 15,
            image: Image::new(width, height),
            filename: filename,
        }
    }

    pub fn penup(&mut self) {
        self.pen_down = false;
    }

    pub fn pendown(&mut self) {
        self.pen_down = true;
    }

    pub fn set_pen_color(&mut self, color: i32) {
        if color < 0 || color > 15 {
            print_error(
                "invalid color",
                &format!("color must be between 0 and 15, got {}", color),
                &["ensure the color value is numeric", "ensure color value is between 0 and 15"],
                true,
            );
        }

        self.pen_color = color;
    }

    fn move_turtle(&mut self, distance: f64) -> Result<(), String> {
        if self.pen_down {
            let (new_x, new_y) = self.image.draw_simple_line(self.x as i32, self.y as i32, self.heading as i32, distance as i32, COLORS[self.pen_color as usize])?;

            self.x = new_x as f64;
            self.y = new_y as f64;
        } else {
            let (new_x, new_y) = get_end_coordinates(self.x as i32, self.y as i32, self.heading as i32, distance as i32);

            self.x = new_x as f64;
            self.y = new_y as f64;
        }

        Ok(())
    }

    pub fn forward(&mut self, distance: f64) {
        match self.move_turtle(distance) {
            Ok(_) => (),
            Err(e) => {
                print_error(
                    "failed to draw line",
                    &e,
                    &["ensure the distance value is numeric", "ensure the distance value is positive"],
                    true,
                );
            }
        }
    }

    pub fn back(&mut self, distance: f64) {
        match self.move_turtle(-distance) {
            Ok(_) => (),
            Err(e) => {
                print_error(
                    "failed to draw line",
                    &e,
                    &["ensure the distance value is numeric", "ensure the distance value is positive"],
                    true,
                );
            }
        }
    }

    pub fn left(&mut self, degrees: f64) {
        self.heading = (self.heading - degrees).rem_euclid(360.0);
    }

    pub fn right(&mut self, degrees: f64) {
        self.heading = (self.heading + degrees).rem_euclid(360.0);
    }

    pub fn set_heading(&mut self, degrees: f64) {
        self.heading = degrees.rem_euclid(360.0);
    }

    pub fn set_x(&mut self, x: f64) {
        if self.pen_down {
            let distance = (x - self.x).abs();
            let heading = if x < self.x { 270 } else { 90 };
            
            match self.image.draw_simple_line(self.x as i32, self.y as i32, heading as i32, distance as i32, COLORS[self.pen_color as usize]) {
                Ok(_) => (),
                Err(e) => {
                    print_error(
                        "failed to draw line",
                        &e,
                        &["ensure the distance value is numeric", "ensure the distance value is positive"],
                        true,
                    );
                }
            }
        }
    
        self.x = x;
    }    

    pub fn set_y(&mut self, y: f64) {
        if self.pen_down {
            let distance = (y - self.y).abs();
            let heading = if y < self.y { 0 } else { 180 };
            
            match self.image.draw_simple_line(self.x as i32, self.y as i32, heading as i32, distance as i32, COLORS[self.pen_color as usize]) {
                Ok(_) => (),
                Err(e) => {
                    print_error(
                        "failed to draw line",
                        &e,
                        &["ensure the distance value is numeric", "ensure the distance value is positive"],
                        true,
                    );
                }
            }
        }
    
        self.y = y;
    }

    pub fn xcor(&self) -> f64 {
        self.x
    }

    pub fn ycor(&self) -> f64 {
        self.y
    }

    pub fn heading(&self) -> f64 {
        self.heading
    }

    pub fn color(&self) -> i32 {
        self.pen_color
    }

    pub fn generate_svg(&self) {
        match self.image.save_svg(self.filename.as_str()).map_err(|e| e.to_string()) {
            Ok(_) => {},
            Err(error) => print_error(
                "failed to generate SVG",
                &format!("{:?}", error),
                &["ensure the output path is correct", "ensure the output path is writable"],
                true,
            ),
        }
    }
}
