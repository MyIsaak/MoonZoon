use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .lang("en-US")
        .title("Paragraph example")
        .append_to_head(
        "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }
        </style>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
