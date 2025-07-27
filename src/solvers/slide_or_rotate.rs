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

use super::{SlideOrRotateVerificationInfoTrait, SolverUiTrait};
use crate::{slint_ui::SlideOrRotateSolverUi, utils::rgba_image_to_slint_image, CaptchaError};
use image::DynamicImage;
use slint::ComponentHandle;
use std::sync::{
    atomic::{AtomicBool, AtomicU32},
    Arc,
};

impl<T: SlideOrRotateVerificationInfoTrait<Input = (DynamicImage, DynamicImage), Output = u32>>
    SolverUiTrait<T> for SlideOrRotateSolverUi
{
    type DataContainer = Arc<AtomicU32>;

    fn new() -> Self {
        SlideOrRotateSolverUi::new().unwrap()
    }

    fn set_captcha_type(&self) {
        self.set_captcha_type(T::get_captcha_type());
    }

    fn create_data_container() -> Self::DataContainer {
        Arc::new(AtomicU32::new(0))
    }

    fn prepare(
        &self,
        (outer_image, inner_image): (DynamicImage, DynamicImage),
        data_container: Self::DataContainer,
        canceled: Arc<AtomicBool>,
    ) {
        let outer_image = rgba_image_to_slint_image(outer_image.to_rgba8());
        let inner_image = rgba_image_to_slint_image(inner_image.to_rgba8());
        self.set_outer_image(outer_image);
        self.set_inner_image(inner_image);
        let self_weak = self.as_weak();
        self.on_verify(move |result| {
            data_container.store(result.round() as u32, std::sync::atomic::Ordering::Relaxed);
            if let Some(self_weak) = self_weak.upgrade() {
                self_weak.hide().unwrap();
            }
        });
        let canceled_ = Arc::clone(&canceled);
        let self_weak = self.as_weak();
        self.on_cancel(move || {
            canceled.store(true, std::sync::atomic::Ordering::Relaxed);
            if let Some(self_weak) = self_weak.upgrade() {
                self_weak.hide().unwrap();
            }
        });
        self.window().on_close_requested(move || {
            canceled_.store(true, std::sync::atomic::Ordering::Relaxed);
            slint::CloseRequestResponse::HideWindow
        });
    }

    fn get_data(data: Self::DataContainer) -> Result<u32, CaptchaError> {
        Ok(data.load(std::sync::atomic::Ordering::Relaxed))
    }
}
