// captcha_solver_ui
// Copyright (C) 2025 worksoup <https://github.com/worksoup/>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use image::{ImageBuffer, Rgba};

pub fn rgba_image_to_slint_image(rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> slint::Image {
    let (width, height) = rgba_image.dimensions();
    let buffer = slint::SharedPixelBuffer::clone_from_slice(rgba_image.as_raw(), width, height);
    slint::Image::from_rgba8(buffer)
}