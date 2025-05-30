use eframe::egui;

#[repr(C)]
pub struct Response {
    response: egui::Response,
}

impl From<egui::Response> for Response {
    fn from(response: egui::Response) -> Self {
        Self {
            response,
        }
    }
}

impl Response {
    pub fn clicked(&self) -> bool {
        self.response.clicked()
    }
    /*
    pub fn clicked_by(&self, button: PointerButton) -> bool {
        self.response.clicked_by(button)
    }
    */
    pub fn secondary_clicked(&self) -> bool {
        self.response.secondary_clicked()
    }
    pub fn long_touched(&self) -> bool {
        self.response.long_touched()
    }
    pub fn middle_clicked(&self) -> bool {
        self.response.middle_clicked()
    }
    pub fn double_clicked(&self) -> bool {
        self.response.double_clicked()
    }
    pub fn triple_clicked(&self) -> bool {
        self.response.triple_clicked()
    }
    /*
    pub fn double_clicked_by(&self, button: PointerButton) -> bool {
        self.response.double_clicked_by(button)
    }
    pub fn triple_clicked_by(&self, button: PointerButton) -> bool {
        self.response.triple_clicked_by(button)
    }
    */
    pub fn enabled(&self) -> bool {
        self.response.enabled()
    }
    pub fn hovered(&self) -> bool {
        self.response.hovered()
    }
    pub fn contains_pointer(&self) -> bool {
        self.response.contains_pointer()
    }
    pub fn has_focus(&self) -> bool {
        self.response.has_focus()
    }
    pub fn gained_focus(&self) -> bool {
        self.response.gained_focus()
    }
    pub fn lost_focus(&self) -> bool {
        self.response.lost_focus()
    }
    pub fn request_focus(&self) {
        self.response.request_focus();
    }
    pub fn surrender_focus(&self) {
        self.response.surrender_focus();
    }
    pub fn drag_started(&self) -> bool {
        self.response.drag_started()
    }
    /*
    pub fn drag_started_by(&self, button: PointerButton) -> bool {
        self.response.drag_started_by(button)
    }
    */
    pub fn dragged(&self) -> bool {
        self.response.dragged()
    }
    /*
    pub fn dragged_by(&self, button: PointerButton) -> bool {
        self.response.dragged_by(button)
    }
    */
    pub fn drag_stopped(&self) -> bool {
        self.response.drag_stopped()
    }
}
