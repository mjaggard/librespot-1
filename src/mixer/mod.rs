use std::borrow::Cow;

pub mod softmixer;

pub trait Mixer {
    fn init(&self);
    fn start(&self);
    fn stop(&self);
    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;
    fn get_stream_editor(&self) -> Option<Box<StreamEditor>>
    {
        None
    }
}

pub trait StreamEditor {
  fn modify_stream<'a>(&self, data: &'a [i16]) -> Cow<'a, [i16]>;
}