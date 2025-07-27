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

use crate::{
    cut_picture,
    slint_ui::{
        ClickCaptchaSolverUi, ClickCaptchaType, SlideOrRotateCaptchaType, SlideOrRotateSolverUi,
        SlintPoint,
    },
    utils::rgba_image_to_slint_image,
    CaptchaError,
};
use image::DynamicImage;
use slint::{ComponentHandle, Model, ModelRc};
use std::sync::{atomic::AtomicBool, Arc};
use yapt::point_2d::Point;

type TriplePoint<T> = (Point<T>, Point<T>, Point<T>);
pub mod click_captcha;
pub mod slide_or_rotate;
pub trait Marker {
    type Input;
    type Output;
    type SolverUi: SolverUiTrait<Self>;
    fn ui_solver(data: Self::Input) -> Result<Self::Output, CaptchaError> {
        Self::SolverUi::ui_solver(data)
    }
}
pub trait ClickCaptchaVerificationInfoTrait: Marker {
    fn get_captcha_type() -> ClickCaptchaType;
    fn set_data(click_captcha_window: &ClickCaptchaSolverUi, data: <Self as Marker>::Input);
    fn points_to_output(points: ModelRc<SlintPoint>) -> Option<<Self as Marker>::Output>;
}
pub trait SlideOrRotateVerificationInfoTrait: Marker {
    fn get_captcha_type() -> SlideOrRotateCaptchaType;
}

impl ClickCaptchaVerificationInfoTrait for MIconClick {
    fn get_captcha_type() -> ClickCaptchaType {
        ClickCaptchaType::IconClick
    }
    fn set_data(click_captcha_window: &ClickCaptchaSolverUi, image: DynamicImage) {
        let icon = cut_picture(&image, Point { x: 0, y: 160 }, Point { x: 84, y: 20 }).to_image();
        let image = cut_picture(&image, Point { x: 0, y: 0 }, Point { x: 320, y: 160 }).to_image();
        let icon = rgba_image_to_slint_image(icon);
        let image = rgba_image_to_slint_image(image);
        click_captcha_window.set_click_icon(icon);
        click_captcha_window.set_image(image);
    }
    fn points_to_output(points: ModelRc<SlintPoint>) -> Option<TriplePoint<u32>> {
        points
            .row_data(0)
            .and_then(|p0| {
                points
                    .row_data(1)
                    .map(|p1| (p0.into_point(), p1.into_point()))
            })
            .and_then(|(p0, p1)| points.row_data(2).map(|p2| (p0, p1, p2.into_point())))
    }
}

impl ClickCaptchaVerificationInfoTrait for MTextClick {
    fn get_captcha_type() -> ClickCaptchaType {
        ClickCaptchaType::TextClick
    }
    fn set_data(
        click_captcha_window: &ClickCaptchaSolverUi,
        (hanzi, image): (String, DynamicImage),
    ) {
        let image = rgba_image_to_slint_image(image.to_rgba8());
        click_captcha_window.set_image(image);
        click_captcha_window.set_hanzi(hanzi.into());
    }
    fn points_to_output(points: ModelRc<SlintPoint>) -> Option<TriplePoint<u32>> {
        MIconClick::points_to_output(points)
    }
}

impl ClickCaptchaVerificationInfoTrait for MObstacle {
    fn get_captcha_type() -> ClickCaptchaType {
        ClickCaptchaType::Obstacle
    }
    fn set_data(click_captcha_window: &ClickCaptchaSolverUi, image: DynamicImage) {
        let icon = cut_picture(&image, Point { x: 0, y: 160 }, Point { x: 20, y: 20 }).to_image();
        let image = cut_picture(&image, Point { x: 0, y: 0 }, Point { x: 320, y: 160 }).to_image();
        let icon = rgba_image_to_slint_image(icon);
        let image = rgba_image_to_slint_image(image);
        click_captcha_window.set_click_icon(icon);
        click_captcha_window.set_image(image);
    }
    fn points_to_output(points: ModelRc<SlintPoint>) -> Option<Point<u32>> {
        points.row_data(0).map(SlintPoint::into_point)
    }
}

impl SlideOrRotateVerificationInfoTrait for MSlide {
    fn get_captcha_type() -> SlideOrRotateCaptchaType {
        SlideOrRotateCaptchaType::Slide
    }
}
impl SlideOrRotateVerificationInfoTrait for MRotate {
    fn get_captcha_type() -> SlideOrRotateCaptchaType {
        SlideOrRotateCaptchaType::Rotate
    }
}

pub trait SolverUiTrait<T: Marker + ?Sized>: Sized + ComponentHandle {
    type DataContainer: Clone;
    fn new() -> Self;
    fn ui_solver(data: <T as Marker>::Input) -> Result<<T as Marker>::Output, CaptchaError> {
        let ui = Self::new();
        let data_container = Self::create_data_container();
        let canceled = Arc::new(AtomicBool::new(false));
        ui.set_captcha_type();
        ui.prepare(data, data_container.clone(), Arc::clone(&canceled));
        let _ = ui.run();
        if canceled.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(CaptchaError::Canceled("操作已取消。".to_owned()));
        }
        Self::get_data(data_container)
    }
    fn set_captcha_type(&self);
    fn create_data_container() -> Self::DataContainer;
    fn prepare(
        &self,
        data: <T as Marker>::Input,
        data_container: Self::DataContainer,
        canceled: Arc<AtomicBool>,
    );
    fn get_data(data_container: Self::DataContainer)
        -> Result<<T as Marker>::Output, CaptchaError>;
}

pub struct MSlide;
impl Marker for MSlide {
    type Input = (DynamicImage, DynamicImage);
    type Output = u32;
    type SolverUi = SlideOrRotateSolverUi;
}
pub struct MRotate;
impl Marker for MRotate {
    type Input = (DynamicImage, DynamicImage);
    type Output = u32;
    type SolverUi = SlideOrRotateSolverUi;
}
pub struct MIconClick;
impl Marker for MIconClick {
    type Input = DynamicImage;
    type Output = (Point<u32>, Point<u32>, Point<u32>);
    type SolverUi = ClickCaptchaSolverUi;
}
pub struct MTextClick;
impl Marker for MTextClick {
    type Input = (String, DynamicImage);
    type Output = (Point<u32>, Point<u32>, Point<u32>);
    type SolverUi = ClickCaptchaSolverUi;
}
pub struct MObstacle;
impl Marker for MObstacle {
    type Input = DynamicImage;
    type Output = Point<u32>;
    type SolverUi = ClickCaptchaSolverUi;
}
