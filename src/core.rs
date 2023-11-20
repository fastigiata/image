use image::{DynamicImage, GenericImageView};

pub struct ImageWrapper {
    /// a wrapper around ImageBuffer that provides dynamic behavior
    dyn_image: DynamicImage,
}

impl ImageWrapper {
    pub fn new(inner: DynamicImage) -> Self {
        Self {
            dyn_image: inner
        }
    }

    pub fn dimension(&self) -> (u32, u32) {
        self.dyn_image.dimensions()
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn t() {}
}