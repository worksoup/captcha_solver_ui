pub mod slint_ui;
pub mod solvers;
pub mod utils;

#[cfg(feature = "cxlib_integrated")]
pub use cxlib_error::CaptchaError;
#[cfg(not(feature = "cxlib_integrated"))]
#[derive(Debug)]
pub enum CaptchaError {
    VerifyFailed,
    Canceled(String),
}
#[cfg(not(feature = "cxlib_integrated"))]
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

#[cfg(feature = "cxlib_integrated")]
pub use cxlib_imageproc::cut_picture;
#[cfg(feature = "cxlib_integrated")]
pub use cxlib_imageproc::Point;
#[cfg(not(feature = "cxlib_integrated"))]
pub use imageproc::point::Point;
#[cfg(not(feature = "cxlib_integrated"))]
pub fn cut_picture<I: image::GenericImageView>(
    picture: &I,
    top_left: Point<u32>,
    wh: Point<u32>,
) -> image::SubImage<&I> {
    image::imageops::crop_imm(picture, top_left.x, top_left.y, wh.x, wh.y)
}