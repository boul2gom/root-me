use std::error::Error;
use std::io::Read;
use flate2::read::ZlibDecoder;
use irc::proto::{Command, Response};
use root_me::client::irc::Irc;

pub static mut READY_TO_HANDLE: bool = false;

/// For IRC challenges, we need to handle the messages sent by the server.
///
/// # Examples
/// ```
/// Irc::new("irc.root-me.org", 6667, "nullptr_rs", "#root-me_challenge").await
///     .with(fourth_challenge)
///     .run().await;
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Irc::new("irc.root-me.org", 6667, "nullptr_rs", "#root-me_challenge").await
        .with(fourth_challenge)
        .run().await;

    Ok(())
}

/*
First, "Go back to college"
Pass: jaimlefr0m4g
 */
pub unsafe fn first_challenge(client: &Irc, command: &Command) {
    match command {
        Command::Response(Response::RPL_WELCOME, ..) => {
            client.client.send_privmsg("candy", " !ep1").expect("Could not send message");
            READY_TO_HANDLE = true;
        }

        Command::PRIVMSG(_, response) => {
            if READY_TO_HANDLE {
                let numbers = response.split(" / ").collect::<Vec<&str>>();
                let left = numbers[0].trim().parse::<f32>().unwrap();
                let right = numbers[1].trim().parse::<f32>().unwrap();

                let result = left.sqrt() * right;
                let formatted = format!("!ep1 -rep {:.2}", result);

                client.client.send_privmsg("candy", formatted).expect("Could not send response message");

                READY_TO_HANDLE = false;
            }
        }

        _ => {}
    }
}

/*
Second, "Encoded string"
Pass: Viv3l"64
 */
pub unsafe fn second_challenge(client: &Irc, command: &Command) {
    match command {
        Command::Response(Response::RPL_WELCOME, ..) => {
            client.client.send_privmsg("candy", " !ep2").expect("Could not send message");
            READY_TO_HANDLE = true;
        }

        Command::PRIVMSG(_, response) => {
            if READY_TO_HANDLE {
                let decoded = base64::decode(response).unwrap();

                let formatted = format!("!ep2 -rep {}", String::from_utf8(decoded).unwrap());
                client.client.send_privmsg("candy", formatted).expect("Could not send response message");

                READY_TO_HANDLE = false;
            }
        }

        _ => {}
    }
}

/*
Third, "The Roman’s wheel"
Pass: 3bienBr4v0Continuepe7i7PONEY
 */
pub unsafe fn third_challenge(client: &Irc, command: &Command) {
    match command {
        Command::Response(Response::RPL_WELCOME, ..) => {
            client.client.send_privmsg("candy", " !ep3").expect("Could not send message");
            READY_TO_HANDLE = true;
        }

        Command::PRIVMSG(_, response) => {
            if READY_TO_HANDLE {
                let decoded = response.chars().map(|char| {
                    match char {
                        'A' ..= 'M' | 'a' ..= 'm' => ((char as u8) + 13) as char,
                        'N' ..= 'Z' | 'n' ..= 'z' => ((char as u8) - 13) as char,
                        _ => char
                    }
                }).collect::<String>();

                let formatted = format!("!ep3 -rep {}", decoded);
                client.client.send_privmsg("candy", formatted).expect("Could not send response message");

                READY_TO_HANDLE = false;
            }
        }

        _ => {}
    }
}

/*
Fourth, "Uncompress me"
Pass: tumasp0wned
 */
pub unsafe fn fourth_challenge(client: &Irc, command: &Command) {
    match command {
        Command::Response(Response::RPL_WELCOME, ..) => {
            client.client.send_privmsg("candy", " !ep4").expect("Could not send message");
            READY_TO_HANDLE = true;
        }

        Command::PRIVMSG(_, response) => {
            if READY_TO_HANDLE {
                let decoded = base64::decode(response).unwrap();
                let mut decoder = ZlibDecoder::new(&decoded[..]);

                let mut decompressed = String::new();
                decoder.read_to_string(&mut decompressed).expect("Could not decompress");

                let formatted = format!("!ep4 -rep {}", decompressed);
                client.client.send_privmsg("candy", formatted).expect("Could not send response message");

                READY_TO_HANDLE = false;
            }
        }

        _ => {}
    }
}