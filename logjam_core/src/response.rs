use eframe::egui;

#[repr(C)]
pub struct Response {
    response: *const egui::Response,
}

impl From<egui::Response> for Response {
    fn from(from: egui::Response) -> Self {
        Self {
            response: &from as *const egui::Response,
        }
    }
}

impl Response {
    pub fn clicked(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.clicked()
    }
    /*
    pub fn clicked_by(&self, button: PointerButton) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.clicked_by(button)
    }
    */
    pub fn secondary_clicked(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.secondary_clicked()
    }
    pub fn long_touched(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.long_touched()
    }
    pub fn middle_clicked(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.middle_clicked()
    }
    pub fn double_clicked(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.double_clicked()
    }
    pub fn triple_clicked(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.triple_clicked()
    }
    /*
    pub fn double_clicked_by(&self, button: PointerButton) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.double_clicked_by(button)
    }
    pub fn triple_clicked_by(&self, button: PointerButton) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.triple_clicked_by(button)
    }
    */
    pub fn enabled(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.enabled()
    }
    pub fn hovered(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.hovered()
    }
    pub fn contains_pointer(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.contains_pointer()
    }
    pub fn has_focus(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.has_focus()
    }
    pub fn gained_focus(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.gained_focus()
    }
    pub fn lost_focus(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.lost_focus()
    }
    pub fn request_focus(&self) {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.request_focus();
    }
    pub fn surrender_focus(&self) {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.surrender_focus();
    }
    pub fn drag_started(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.drag_started()
    }
    /*
    pub fn drag_started_by(&self, button: PointerButton) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.drag_started_by(button)
    }
    */
    pub fn dragged(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.dragged()
    }
    /*
    pub fn dragged_by(&self, button: PointerButton) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.dragged_by(button)
    }
    */
    pub fn drag_stopped(&self) -> bool {
        let response = unsafe { self.response.as_ref().unwrap() };
        response.drag_stopped()
    }
}
