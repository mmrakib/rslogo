use unsvg::{Image, COLORS, get_end_coordinates};

pub struct Turtle {
    x: f64,
    y: f64,
    heading: f64,
    pen_down: bool,
    pen_color: u32,
    image: unsvg::Image,
}

impl Turtle {
    pub fn new(width: u32, height: u32) -> Self {
        Turtle {
            x: (width / 2) as f64,
            y: (height / 2) as f64,
            heading: 0.0,
            pen_down: false,
            pen_color: 15,
            image: Image::new(width, height),
        }
    }

    pub fn penup(&mut self) {
        self.pen_down = false;
    }

    pub fn pendown(&mut self) {
        self.pen_down = true;
    }

    pub fn set_pen_color(&mut self, color: u32) {
        if color <= 0 || color >= 15 {
            panic!("Invalid pen color");
        }

        self.pen_color = color;
    }

    fn move_turtle(&mut self, distance: f64) -> Result<(), String> {
        if self.pen_down {
            let (new_x, new_y) = self.image.draw_simple_line(self.x as i32, self.y as i32, self.heading as i32, distance as i32, COLORS[self.pen_color as usize])?;

            self.x = new_x as f64;
            self.y = new_y as f64;
        } else {
            let (new_x, new_y) = unsvg::get_end_coordinates(self.x as i32, self.y as i32, self.heading as i32, distance as i32);

            self.x = new_x as f64;
            self.y = new_y as f64;
        }

        Ok(())
    }

    pub fn forward(&mut self, distance: f64) -> Result<(), String> {
        self.move_turtle(distance);

        Ok(())
    }

    pub fn back(&mut self, distance: f64) -> Result<(), String> {
        self.move_turtle(-distance);

        Ok(())
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

    pub fn set_x(&mut self, x: f64) -> Result<(), String> {
        if self.pen_down {
            let distance = (x - self.x).abs();
            let heading = if x < self.x { 270 } else { 90 };
            
            self.image.draw_simple_line(self.x as i32, self.y as i32, heading as i32, distance as i32, COLORS[self.pen_color as usize])?;
        }
    
        self.x = x;
    
        Ok(())
    }    

    pub fn set_y(&mut self, y: f64) -> Result<(), String> {
        if self.pen_down {
            let distance = (y - self.y).abs();
            let heading = if y < self.y { 0 } else { 180 };
            
            self.image.draw_simple_line(self.x as i32, self.y as i32, heading as i32, distance as i32, COLORS[self.pen_color as usize])?;
        }
    
        self.y = y;
    
        Ok(())
    }    

    pub fn generate_svg(&self, filename: &str) -> Result<(), String> {
        self.image.save_svg(filename).map_err(|e| e.to_string())?;

        Ok(())
    }
}
