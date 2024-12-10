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
    top_left: yapt::Point<u32>,
    wh: yapt::Point<u32>,
) -> image::SubImage<&I> {
    image::imageops::crop_imm(picture, top_left.x, top_left.y, wh.x, wh.y)
}
