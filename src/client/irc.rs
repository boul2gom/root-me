use futures::StreamExt;
use irc::client::{Client, ClientStream};
use irc::client::data::Config;
use irc::proto::Command;

pub type Handler = unsafe fn(&Irc, &Command);

pub struct Irc {
    pub client: Client,
    pub stream: ClientStream,

    pub functions: Vec<Handler>
}

impl Irc {
    pub async fn new(server: &str, port: u16, name: &str, channel: &str) -> Self {
        let config = Config {
            server: Some(server.to_owned()),
            port: Some(port),
            nickname: Some(name.to_owned()),
            realname: Some(name.to_owned()),
            channels: vec![channel.to_owned()],
            use_tls: Some(false),
            ..Config::default()
        };

        let mut client = Client::from_config(config).await.expect("Could not create client");
        client.identify().expect("Could not identify");

        let stream = client.stream().expect("Could not get stream");

        Self {
            client,
            stream,
            functions: Vec::new()
        }
    }

    pub fn with(mut self, function: Handler) -> Self {
        self.functions.push(function);
        self
    }

    pub async fn run(&mut self) {
        println!("Be careful, there is {} handlers registered", self.functions.len());
        while let Some(message) = self.stream.next().await.transpose().expect("Failed to get message") {
            let command = &message.command;
            println!("{:?}", &command);

            for handler in &self.functions {
                unsafe {
                    handler(self, &command);
                }
            }
        }
    }
}