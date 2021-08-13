extern crate image;

pub mod imaging;



/// A module for defining shapes to be drawn on a canvas
pub mod shapes {
    use super::*;
    use image::Rgb;

    pub struct Sphere {
        pub center: [i64; 3],
        pub radius: u64,
        pub color: Rgb<u8>,
    }

    impl Sphere {
        /// Creates new Sphere, with its center at (0, 0, 0), a radius of 1 unit, and color of white
        pub fn new() -> Sphere {
            Sphere {
                center: [0, 0, 0],
                radius: 1,
                color: Rgb([255, 255, 255]),
            }
        }

        /// Choose your specs
        pub fn from(center: [i64; 3], radius: u64, color: Rgb<u8>) -> Sphere {
            Sphere {
                center,
                radius,
                color,
            }
        }
    }

}

#[cfg(test)]
pub mod tests {
    use crate::shapes::Sphere;
    use crate::image::Rgb;
    use crate::imaging::canvas::{Canvas, Point};

    #[test]
    fn build_canvas_test() {
        let c: Canvas = Canvas::new();
        assert_eq!(512, c.get_width());
        assert_eq!(512, c.get_height());
        let c: Canvas = Canvas::from(1920, 1080);
        assert_eq!(1920, c.get_width());
        assert_eq!(1080, c.get_height());
    }

    #[test]
    fn convert_xy_test() {
        let c: Canvas = Canvas::new();
        let values = c.convert_xy(256, 256);
        assert_eq!(512, values.0);
        assert_eq!(512, values.1);
        let other_values = c.convert_xy(0, 0);
        assert_eq!(256, other_values.0);
        assert_eq!(256, other_values.1);
        let negative_values = c.convert_xy(-256, -256);
        assert_eq!(0, negative_values.0);
        assert_eq!(0, negative_values.1);
    }

    #[test]
    fn create_empty_sphere() {
        let s: Sphere = Sphere::new();
        assert_eq!([0, 0, 0], s.center);
        assert_eq!(1, s.radius);
        assert_eq!(Rgb([255, 255, 255]), s.color);
    }

    #[test]
    fn create_sphere() {
        let s: Sphere = Sphere::from([3, 3, 4], 2, Rgb([255, 0, 255]));
        assert_eq!([3, 3, 4], s.center);
        assert_eq!(2, s.radius);
        assert_eq!(Rgb([255, 0, 255]), s.color);
    }

    #[test]
    fn create_point() {
        let p: Point = Point::new();
        assert_eq!([0, 0, 0], p.coordinates());

    }
}
