use crate::{error, GenericImageView};

/// A Rectangle defined by its top left corner, width and height.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The x coordinate of the top left corner.
    pub x: u32,
    /// The y coordinate of the top left corner.
    pub y: u32,
    /// The rectangle's width.
    pub width: u32,
    /// The rectangle's height.
    pub height: u32,
}

impl Rect {
    /// Construct a rectangle representing an image with its top-left corner.
    pub(crate) fn from_image_at(image: &(impl GenericImageView + ?Sized), x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            width: image.width(),
            height: image.height(),
        }
    }

    pub(crate) fn test_in_bounds(
        &self,
        image: &(impl GenericImageView + ?Sized),
    ) -> Result<(), error::ImageError> {
        if image.width().checked_sub(self.width) >= Some(self.x)
            && image.height().checked_sub(self.height) >= Some(self.y)
        {
            Ok(())
        } else {
            Err(error::ImageError::Parameter(
                error::ParameterError::from_kind(error::ParameterErrorKind::DimensionMismatch),
            ))
        }
    }
}
