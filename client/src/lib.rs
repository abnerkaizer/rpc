use clap::{App, Arg};
use service::WorldClient;
use std::{io, net::SocketAddr};
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let flags = App::new("Client")
        .version("0.1")
        .author("Abner Kaizer <abnerkaizer@protonmail.com>")
        .about("Output a 'Hello, <NAME> !' or ")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Sets the user name to be displayed.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("Set the filename to be read.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Set the filename to be written.")
                .takes_value(true),
        )
        .get_matches();

    let server_addr = String::from("127.0.0.1:9090");
    let server_addr = server_addr
        .parse::<SocketAddr>()
        .unwrap_or_else(|e| panic!(r#"--server_addr value "{}" invalid: {}"#, server_addr, e));

    let name = match flags.value_of("name") {
        Some(name) => name.to_string(),
        None => "".to_string(),
    };

    let input = match flags.value_of("input") {
        Some(input) => input.to_string(),
        None => "".to_string(),
    };

    let output = match flags.value_of("output") {
        Some(output) => output.to_string(),
        None => "".to_string(),
    };

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let mut client = WorldClient::new(client::Config::default(), transport.await?).spawn()?;

    // The client has an RPC method for each RPC defined in the annotated trait. It takes the same
    // args as defined, with the addition of a Context, which is always the first arg. The Context
    // specifies a deadline and trace information which can be helpful in debugging requests.
    if name != "" {
        let hello = client.hello(context::current(), name).await?;
        println!("{}", hello);
    }

    if input != "" && output != "" {
        let contents = client.read_file(context::current(), input).await?;
        client
            .write_file(context::current(), output, contents)
            .await?;
    } else if input == "" && output == "" {
    } else {
        println!("Need both input and output files.");
    }

    Ok(())
}
pub fn run() {
    match main(){
        Ok(r) => r,
        Err(_e) => ()
    }
}
