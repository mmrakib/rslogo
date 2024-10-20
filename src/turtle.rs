use unsvg::{Image, COLORS};

#[derive(Debug)]
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
            x: 0.0,
            y: 0.0,
            heading: 0.0,
            pen_down: false,
            pen_color: 15,
            image: Image::new(width, height),
        }
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn set_pen_color(&mut self, color: u32) {
        self.pen_color = color;
    }

    fn move_turtle(&mut self, distance: f64) -> Result<(), String> {
        let radians = self.heading.to_radians();
        let new_x = self.x + distance * radians.cos();
        let new_y = self.y - distance * radians.sin(); // The y-axis in SVG is oriented downwards

        if self.pen_down {
            self.image.draw_simple_line(self.x, self.y, new_x, new_y, COLORS[self.pen_color])?;
        }

        self.x = new_x;
        self.y = new_y;
    }

    pub fn forward(&mut self, distance: f64) {
        self.move_turtle(distance);
    }

    pub fn back(&mut self, distance: f64) {
        self.move_turtle(-distance);
    }

    pub fn left(&mut self, distance: f64) {
        self.heading = (self.heading - degrees) % 360.0;
    }

    pub fn right(&mut self, distance: f64) {
        self.heading = (self.heading + degrees) % 360.0;
    }

    pub fn set_heading(&mut self, degrees: f64) {
        self.heading = degrees % 360.0;
    }

    pub fn set_x(&mut self, x: f64) -> Result<(), String> {
        if self.pen_down {
            self.image.draw_simple_line(self.x, self.y, x, self.y, COLORS[self.pen_color])?;
        }

        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) -> Result<(), String> {
        if self.pen_down {
            self.iamge.draw_simple_line(self.x, self.y, self.x, y, COLORS[self.pen_color])?;
        }

        self.y = y;
    }

    pub fn generate_svg(&self, filename: &str) -> Result<(), String> {
        image.save_svg(filename)?;
    }
}
