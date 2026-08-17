/*! # The built-in rasterizers.

  Plotters make a minimal backend ability assumption - which is drawing a pixel on
  backend. And this is the rasterizer that utilize this minimal ability to build a
  fully functioning backend.

*/

// TODO: We need to revisit this. It has been a long time since last time we figured out
// the question mark operator has a huge performance impact due to LLVM unable to handle it.
// So the question is if this trick is still useful, or LLVM is smart enough to handle it since
// then.
//
// --
// Original comment:
//
// ? operator is very slow. See issue #58 for details
macro_rules! check_result {
    ($e:expr) => {
        let result = $e;
        #[allow(clippy::question_mark)]
        if result.is_err() {
            return result;
        }
    };
}

mod line;
pub use line::draw_line;

mod rect;
pub use rect::draw_rect;

mod circle;
pub use circle::draw_circle;

mod polygon;
pub use polygon::fill_polygon;

mod path;
pub use path::polygonize;
