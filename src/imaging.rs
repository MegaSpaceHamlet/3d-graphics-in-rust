pub mod canvas {
    use image::{ImageBuffer, RgbImage, Rgb, ImageResult};

    /// A wrapper for ImageBuffer 
    pub struct Canvas {
        /// The img is the core of the Canvas. 
        /// The width and height arre of type u32, because they represent a 
        /// positive real number.
        img: RgbImage,
        width: u32,
        height: u32,
    }


    impl Canvas {
        /// Default width and height is 512
        pub fn new() -> Canvas {
            let default_value: u32 = 512;
            Canvas {
                width: default_value,
                height: default_value,
                img: ImageBuffer::new(default_value, default_value),
            }
        }

        /// Supply your own dimensions
        pub fn from(width: u32, height: u32) -> Canvas {
            Canvas {
                width, 
                height,
                img: ImageBuffer::new(width, height),
            }
        }

        /// Returns the width and height of a canvas as a tuple
        pub fn dimensions(&self) -> (u32, u32) {
            (self.width, self.height)
        }

        /// Get the width 
        pub fn get_width(&self) -> u32 {
            self.width
        }

        /// Get the height
        pub fn get_height(&self) -> u32 {
            self.height
        }

        /// Returns put_pixel ready co-ordinates. From (0, 0) being in the center to (0, 0)
        /// at the top left.  
        ///
        /// # Arguments
        ///
        /// * 'cx' - An i64 that represents a point on the x coordinate plane using (0, 0) at the center. It is an unsigned integer 
        /// because it spans the canvas from -w until +w.The function will convert the points to u32 for the ImageBuffer's coordinate system.
        /// * 'cy' - The same, but for y
        pub fn convert_xy(&self, cx: i64, cy: i64) -> (u32, u32) {
            let sx: i64;
            let sy: i64;
            sx = self.width as i64 / 2 + cx;
            sy = self.width as i64 / 2 + cy;

            ((sx as u32).into(), (sy as u32).into())
        }

        /// Puts pixel after converting to (0, 0) top-left
        /// 
        /// # Arguements
        /// 
        /// * 'x' - x-coordinate on canvas plane
        /// * 'y' - y-coordinate on canvas plane
        /// * 'color' - an array of u8 values of length 3. Inserted into Rgb in function as color. 
        /// TODO: Make function take Rgb directly instead?
        pub fn put_pixel(&mut self, x: i64, y: i64, color: [u8; 3]) {
            let values = self.convert_xy(x, y);
            self.img.put_pixel(values.0, values.1, Rgb(color));
        }

        /// Wrapper for ImageBuffer::save(). Returns the result to caller
        /// 
        /// # Arguements
        /// 
        /// * 'filename' - Path where you want the file to be saved
        pub fn save(&self, filename: &str) -> ImageResult<()> {
            self.img.save(filename) 
        }
    }

    /// Point in a 3-d space (x, y, z)
    /// TODO: Create macro rule for defining Point
    pub struct Point {
        pub x: i64,
        pub y: i64,
        pub z: i64
    }

    impl Point {

        /// Creates new Point at (0, 0, 0)
        pub fn new() -> Point {
            Point {
                x: 0,
                y: 0,
                z: 0
            }
        }

        /// Create new Point at desired location using 3 separate values
        /// TODO: Make function capable of receiving any type that implements 'Primitive'. https://docs.rs/image/0.18.0/image/trait.Primitive.html
        /// 
        /// # Arguements
        /// 
        /// * x - point on x-axis
        /// * y - point on y-axis
        /// * z - point on y-axis 
        pub fn from(x: i64, y: i64, z: i64) -> Point {
            Point {
                x,
                y, 
                z
            }
        }


        /// Create new Point from array
        /// 
        /// # Arguements
        /// 
        /// * coordinates - array holding 3 values representing, x, y, and z
        pub fn from_array(coordinates: [i64; 3]) -> Point {
            Point {
                x: coordinates[0],
                y: coordinates[1],
                z: coordinates[2]
            }
        }

        /// Return coordinates as array in form (x, y, z)
        pub fn coordinates(&self) -> [i64; 3] {
            [self.x, self.y, self.z]
        }
    }
}

pub mod painting {
    
}
