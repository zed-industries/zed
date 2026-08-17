pub trait OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32);
    fn line_to(&mut self, x: f32, y: f32);
    fn curve_to(&mut self, cp0x: f32, cp0y: f32, cp1x: f32, cp1y: f32, x: f32, y: f32);
    fn close(&mut self);
}
