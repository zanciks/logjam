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
    /*
    pub fn drag_stopped_by(&self, button: PointerButton) -> bool {
        self.response.drag_stopped_by(button)
    }
    pub fn drag_delta(&self) -> Vec2 {
        self.response.drag_delta()
    }
    pub fn drag_motion(&self) -> Vec2 {
        self.response.drag_motion()
    }
    pub fn dnd_set_drag_payload<Payload: Any + Send + Sync>(&self, payload: Payload) {
        self.response.dnd_set_drag_payload(payload)
    }
    pub fn dnd_hover_payload<Payload: Any + Send + Sync>(&self) -> Option<Arc<Payload>> {
        self.response.dnd_hover_payload(payload)
    }
    pub fn dnd_release_payload<Payload: Any + Send + Sync>(&self) -> Option<Arc<Payload>> {
        self.response.dnd_release_payload(payload)
    }
    pub fn interact_pointer_pos(&self) -> Option<Pos2> {
        self.response.interact_pointer_pos()
    }
    pub fn hover_pos(&self) -> Option<Pos2> {
        self.response.hover_pos()
    }
    */
    pub fn is_pointer_button_down_on(&self) -> bool {
        self.response.is_pointer_button_down_on()
    }
    pub fn changed(&self) -> bool {
        self.response.changed() // will this even work? needs to be tested
    }
    pub fn mark_changed(&mut self) {
        self.response.mark_changed();
    }
}
