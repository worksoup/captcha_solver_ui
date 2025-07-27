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
            CaptchaError::VerifyFailed => f.write_str("验证失败。"),
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
