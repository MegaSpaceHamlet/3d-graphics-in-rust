pub mod imaging;

#[cfg(test)]
pub mod tests {
    use crate::imaging::canvas::Canvas;

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
}

