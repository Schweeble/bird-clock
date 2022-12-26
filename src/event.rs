use crate::models::bird::Bird;

pub enum Event {
    GetBird,
    GotBird(Bird),
    PlaySound(Bird),
    StopSound,
}