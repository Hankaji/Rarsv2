use chrono::{DateTime as DT, Local};
use iced::{time, widget::text, Element, Subscription};

#[derive(Default)]
pub struct Clock {
    time_now: DT<Local>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick(DT<Local>),
}

impl Clock {
    pub fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_millis(1000)).map(|_| Message::Tick(Local::now()))
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Tick(local_time) => self.time_now = local_time,
        }
    }

    pub fn view(&self) -> Element<Message> {
        text(self.time_now.format("%A, %b %d %H:%M").to_string()).into()
    }
}
