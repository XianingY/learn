use crate::{db::Db, connection::Connection, command::Command};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, error};

pub async fn run(listener: TcpListener) -> crate::Result<()> {
    let db = Db::new();

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            if let Err(err) = handle_connection(socket, db).await {
                error!("connection error: {}", err);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream, db: Db) -> crate::Result<()> {
    let mut conn = Connection::new(socket);

    while let Some(frame_owned) = conn.read_frame().await? {
        info!("received frame: {:?}", frame_owned);
        let cmd = Command::from_frame_owned(frame_owned)?;
        cmd.apply(&db, &mut conn).await?;
    }

    Ok(())
}
