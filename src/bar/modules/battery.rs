use battery::{Service, ServiceBlocking};
use futures::{SinkExt, StreamExt};
use iced::{stream::channel, Subscription};

use crate::widgets::{Element, Text};

pub struct Battery {
    percentage: f32,
}

#[derive(Clone, Debug)]
pub enum Message {
    PercentageChange(f32),
}

impl Default for Battery {
    fn default() -> Self {
        let bat = ServiceBlocking::init().unwrap();
        if let Some(bat1) = bat.devices.first() {
            let percentage = bat1.percentage().unwrap() as f32;
            Self { percentage }
        } else {
            Self { percentage: -1.0 }
        }
    }
}

impl Battery {
    pub fn subscription(&self) -> Subscription<Message> {
        // TODO: This feel quite janky rn, will need to look if theres a better way
        Subscription::run(|| {
            channel(1, |mut output| async move {
                let bat = Service::init().await.unwrap();
                if let Some(bat1) = bat.devices.first() {
                    let mut percent_stream = bat1.receive_percentage_changed().await;
                    while let Some(percentage) = percent_stream.next().await {
                        println!("Property changed");
                        output
                            .send(Message::PercentageChange(
                                percentage.get().await.unwrap() as f32
                            ))
                            .await
                            .unwrap();
                    }

                    println!("If it reached here that means shit no longer waiting for percentage change")
                }
            })
        })
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            // Message::Tick(local_time) => self.time_now = local_time,
            Message::PercentageChange(p) => self.percentage = p,
        }
    }

    pub fn view(&self) -> Element<Message> {
        // Text::new(self.time_now.format("%A, %b %d %H:%M").to_string()).into()
        Text::new(format!("{}%", self.percentage)).into()
    }
}
