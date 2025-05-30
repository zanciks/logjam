use eframe::egui::{self, Response, WidgetText};

#[repr(C)]
pub struct Ui {
    ui: *mut egui::Ui,
}

impl From<&mut egui::Ui> for Ui {
    fn from(ui: &mut egui::Ui) -> Self {
        Self { ui: ui as *mut egui::Ui }
    }
}

impl Ui {
    pub fn add(&mut self, _widget: impl egui::Widget) -> Response {
        todo!()
    }
    pub fn add_sized(&mut self, _max_size: impl Into<egui::Vec2>, _widget: impl egui::Widget) -> Response {
        todo!()
    }
    pub fn put(&mut self, _max_rect: egui::Rect, _widget: impl egui::Widget) -> Response {
        todo!()
    }
    pub fn add_enabled(&mut self, _enabled: bool, _widget: impl egui::Widget) -> Response {
        todo!()
    }
    pub fn add_enabled_ui<R>(&mut self, _enabled: bool, _add_contents: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        todo!()
    }
    pub fn add_visible(&mut self, _visible: bool, _widget: impl egui::Widget) -> Response {
        todo!()
    }
    pub fn add_visible_ui<R>(&mut self, _visible: bool, _add_contents: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        todo!()
    }
    pub fn add_space(&mut self, _amount: f32) {
        todo!()
    }
    pub fn label(&mut self, text: impl Into<WidgetText>) -> Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.label(text).into()
    }
    pub fn colored_label(&mut self, _color: impl Into<egui::Color32>, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn heading(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn monospace(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn code(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn small(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn strong(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn weak(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn link(&mut self, _text: impl Into<egui::RichText>) -> Response {
        todo!()
    }
    pub fn hyperlink(&mut self, _url: impl ToString) -> Response {
        todo!()
    }
    pub fn hyperlink_to(&mut self, _label: impl Into<WidgetText>, _url: impl ToString) -> Response {
        todo!()
    }
    pub fn text_edit_singleline<S: egui::TextBuffer>(&mut self, _text: &mut S) -> Response {
        todo!()
    }
    pub fn text_edit_multiline<S: egui::TextBuffer>(&mut self, _text: &mut S) -> Response {
        todo!()
    }
    pub fn code_editor<S: egui::TextBuffer>(&mut self, _text: &mut S) -> Response {
        todo!()
    }
    pub fn button(&mut self, text: impl Into<WidgetText>) -> Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.button(text).into()
    }
    pub fn small_button(&mut self, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn checkbox(&mut self, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn toggle_value(&mut self, _selected: &mut bool, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn radio(&mut self, _selected: bool, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn radio_value<Value: PartialEq>(&mut self, _current_value: &mut Value, _alternative: Value, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn selectable_label(&mut self, _checked: bool, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn selectable_value<Value: PartialEq>(&mut self, _current_value: &mut Value, _selected_value: Value, _text: impl Into<WidgetText>) -> Response {
        todo!()
    }
    pub fn separator(&mut self) -> Response {
        todo!()
    }
    pub fn spinner(&mut self) -> Response {
        todo!()
    }
    pub fn drag_angle(&mut self, _radians: &mut f32) -> Response {
        todo!()
    }
    pub fn drag_angle_tau(&mut self, _radians: &mut f32) -> Response {
        todo!()
    }
    pub fn image<'a>(&mut self, _source: impl Into<egui::ImageSource<'a>>) -> Response {
        todo!()
    }
}