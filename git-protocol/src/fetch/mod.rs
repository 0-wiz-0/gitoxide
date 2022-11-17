mod arguments;
pub use arguments::Arguments;

///
pub mod command;
pub use command::Command;

/// The name of the `git` client in a format suitable for presentation to a `git` server, using `name` as user-defined portion of the value.
pub fn agent(name: impl Into<String>) -> String {
    let mut name = name.into();
    if !name.starts_with("git/") {
        name.insert_str(0, "git/");
    }
    name
}

///
pub mod delegate;
#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub use delegate::Delegate;
pub use delegate::{Action, DelegateBlocking, LsRefsAction};

mod error;
pub use error::Error;
///
pub mod refs;
pub use refs::function::refs;
///
pub mod response;
pub use response::Response;

mod handshake;
pub use handshake::upload_pack as handshake;

/// Send a message to indicate the remote side that there is nothing more to expect from us, indicating a graceful shutdown.
#[maybe_async::maybe_async]
pub async fn indicate_end_of_interaction(
    mut transport: impl git_transport::client::Transport,
) -> Result<(), git_transport::client::Error> {
    // An empty request marks the (early) end of the interaction. Only relevant in stateful transports though.
    if transport.connection_persists_across_multiple_requests() {
        transport
            .request(
                git_transport::client::WriteMode::Binary,
                git_transport::client::MessageKind::Flush,
            )?
            .into_read()
            .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
