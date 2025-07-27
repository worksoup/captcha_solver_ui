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
