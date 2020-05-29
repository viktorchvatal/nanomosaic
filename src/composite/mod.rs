pub struct CompositeState { 
    gui: Option<GlibSender<GuiMessage>>,    
}

impl MessageReceiver<CompositeMessage> for CompositeState {
    fn receive(&mut self, message: CompositeMessage) -> Result<(), String> {
    match message {
        CompositeMessage::InitGui(channel) => Ok(self.init_gui(channel)),
    }
}