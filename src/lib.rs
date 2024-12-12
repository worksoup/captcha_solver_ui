// MIT License
//
// Copyright (c) 2024 worksoup <https://github.com/worksoup/>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod slint_ui;
pub mod solvers;
pub mod utils;

#[derive(Debug)]
pub enum CaptchaError {
    VerifyFailed,
    Canceled(String),
}
impl std::fmt::Display for CaptchaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CaptchaError::VerifyFailed {} => f.write_str("验证失败。"),
            CaptchaError::Canceled(msg) => {
                write!(f, "操作被主动取消：`{msg}`.")
            }
        }
    }
}

pub fn cut_picture<I: image::GenericImageView>(
    picture: &I,
    top_left: yapt::point_2d::Point<u32>,
    wh: yapt::point_2d::Point<u32>,
) -> image::SubImage<&I> {
    image::imageops::crop_imm(picture, top_left.x, top_left.y, wh.x, wh.y)
}
