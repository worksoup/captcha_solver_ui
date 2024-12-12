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
