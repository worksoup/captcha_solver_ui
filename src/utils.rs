use image::{ImageBuffer, Rgba};

pub fn rgba_image_to_slint_image(rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> slint::Image {
    let (width, height) = rgba_image.dimensions();
    let buffer = slint::SharedPixelBuffer::clone_from_slice(rgba_image.as_raw(), width, height);
    slint::Image::from_rgba8(buffer)
}