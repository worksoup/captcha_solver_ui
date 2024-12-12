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

use super::{ClickCaptchaVerificationInfoTrait, Marker, SolverUiTrait};
use crate::slint_ui::{ClickCaptchaSolverUi, SlintPoint};
use crate::CaptchaError;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::sync::{atomic::AtomicBool, Arc};

impl<T: ClickCaptchaVerificationInfoTrait> SolverUiTrait<T> for ClickCaptchaSolverUi {
    type DataContainer = ModelRc<SlintPoint>;

    fn new() -> Self {
        ClickCaptchaSolverUi::new().unwrap()
    }

    fn set_captcha_type(&self) {
        self.set_captcha_type(T::get_captcha_type());
    }

    fn create_data_container() -> Self::DataContainer {
        let points = VecModel::from_iter(Vec::new());
        ModelRc::new(points)
    }

    fn prepare(
        &self,
        data: <T as Marker>::Input,
        data_container: Self::DataContainer,
        canceled: Arc<AtomicBool>,
    ) {
        T::set_data(self, data);
        self.set_points(data_container);
        let self_weak = self.as_weak();
        self.on_verify_button_clicked(move || {
            if let Some(self_weak) = self_weak.upgrade() {
                self_weak.hide().unwrap();
            }
        });
        fn model_rc_to_model<T>(rc: ModelRc<T>) -> Option<std::rc::Rc<dyn Model<Data = T>>> {
            rc.try_into().ok()
        }
        fn model_to_vec_model<T: 'static>(
            model: Option<&std::rc::Rc<dyn Model<Data = T>>>,
        ) -> Option<&VecModel<T>> {
            match model {
                Some(model) => model.as_any().downcast_ref(),
                None => None,
            }
        }
        let points = model_rc_to_model(self.get_points());
        self.on_push_point(move |x, y| {
            let points = model_to_vec_model(points.as_ref());
            if let Some(points) = points {
                if points.row_count() < 3 {
                    points.push(SlintPoint { x, y });
                }
            }
        });
        let points = model_rc_to_model(self.get_points());
        self.on_pop_point(move || {
            let points = model_to_vec_model(points.as_ref());
            if let Some(points) = points {
                let len = points.row_count();
                if len != 0 {
                    let _data = points.remove(len - 1);
                }
            }
        });
        let canceled_ = Arc::clone(&canceled);
        let self_weak = self.as_weak();
        self.on_cancel(move || {
            canceled.store(true, std::sync::atomic::Ordering::Relaxed);
            if let Some(main_window) = self_weak.upgrade() {
                main_window.hide().unwrap();
            }
        });
        self.window().on_close_requested(move || {
            canceled_.store(true, std::sync::atomic::Ordering::Relaxed);
            slint::CloseRequestResponse::HideWindow
        });
    }

    fn get_data(
        data_container: Self::DataContainer,
    ) -> Result<<T as Marker>::Output, CaptchaError> {
        if let Some(data) = T::points_to_output(data_container) {
            Ok(data)
        } else {
            Err(CaptchaError::Canceled("图标选择未完成。".to_owned()))
        }
    }
}
