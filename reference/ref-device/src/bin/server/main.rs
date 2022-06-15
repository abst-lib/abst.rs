use std::iter::Filter;
use std::net::{IpAddr, SocketAddr};
use tokio::sync::mpsc::{channel, Receiver};
use iced::{alignment, Application, Color, Column, Command, Container, Element, Length, Row, Scrollable, Settings, Subscription, Text};
use iced::window::Mode;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use abst_rs::a_sync::{ConnectedDevice, ConnectedDeviceType, Server};
use abst_rs::a_sync::tokio_abst::server::new_tokio_server;
use abst_rs::encryption::NoEncryption;
use abst_rs::protocol::DeviceToDevice;
use crate::alignment::Alignment;

#[tokio::main]
async fn main() {
    let socket = TcpListener::bind("127.0.0.1:3695").await.unwrap();
    let mut server = new_tokio_server(socket);
    let (sender, reciever) = channel::<ServerMessage>(1024);
    tokio::spawn(async move {

        println!("HEY");

        loop {
            let device = server.accept().await.unwrap();
            println!("New Connection");

            let string = match device.connected_type {
                ConnectedDeviceType::Realm => {
                    "Realm".to_string()
                }
                ConnectedDeviceType::DeviceToDevice(ip) => {
                    format!("{}", ip.to_string())
                }
            };
            sender.send(ServerMessage::NewConnection(string)).await;
        }
    });
    ServerView::run(Settings::with_flags(reciever)).unwrap();
}

#[derive(Debug, Clone)]
pub enum GUIMessage {
    Refresh(Result<(), ()>),
}

enum ServerMessage {
    NewConnection(String),
}

enum ServerView<> {
    Loading,
    Loaded(State),
}

struct State {
    channel: Receiver<ServerMessage>,
    connected_devices: Vec<String>,
}

impl State {
    fn new(receiver: Receiver<ServerMessage>) -> ServerView {
        ServerView::Loaded(State {
            channel: receiver,
            connected_devices: vec![],
        })
    }
}

impl Application for ServerView {
    type Executor = iced::executor::Default;
    type Message = GUIMessage;
    type Flags = Receiver<ServerMessage>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (ServerView::Loaded(State {
            channel: flags,
            connected_devices: vec![],
        }), Command::none())
    }

    fn title(&self) -> String {
        format!("One day we will be together")
    }

    fn update(&mut self, message: GUIMessage) -> Command<GUIMessage> {
        match self {
            ServerView::Loading => {
                Command::none()
            }
            ServerView::Loaded(state) => {
                if let Ok(value) = state.channel.try_recv() {
                    match value {
                        ServerMessage::NewConnection(connected_device) => {
                            state.connected_devices.push(connected_device);
                            return Command::perform(async {
                                Ok(())
                            },GUIMessage::Refresh);
                        }
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<GUIMessage> {
        match self {
            ServerView::Loading => {
                Text::new("Loading...").size(40).into()
            }
            ServerView::Loaded(value) => {
                let title = Text::new("Connections")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(alignment::Horizontal::Center);


                let connections: Element<_> = if value.connected_devices.len() > 0 {
                    value.connected_devices
                        .iter_mut()
                        .enumerate()
                        .fold(Column::new().spacing(20), |column, (i, task)| {
                            column.push(view(task))
                        })
                        .into()
                } else {
                    empty_message("No Connections")
                };

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(connections);
                Container::new(content)
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}

pub fn view(value: &str) -> Element<GUIMessage> {
    Row::new()
        .spacing(20)
        .align_items(Alignment::Center)
        .push(Text::new(value).size(20).color(Color::BLACK))
        .into()
}

fn empty_message(message: &str) -> Element<GUIMessage> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .color([0.7, 0.7, 0.7]),
    )
        .width(Length::Fill)
        .height(Length::Units(200))
        .center_y()
        .into()
}