use mini_redis::{Result, frame::FrameOwned, connection::Connection};
use tokio::net::TcpStream;
use bytes::Bytes;

#[tokio::test]
async fn test_ping_set_get() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut conn = Connection::new(stream);

    // Test PING
    conn.write_frame(&FrameOwned::Array(Some(vec![
        FrameOwned::BulkString(Some(Bytes::from("PING"))),
    ]))).await?;
    let resp = conn.read_frame().await?.unwrap();
    assert_eq!(format!("{:?}", resp), "SimpleString(\"PONG\")");

    // Test SET
    conn.write_frame(&FrameOwned::Array(Some(vec![
        FrameOwned::BulkString(Some(Bytes::from("SET"))),
        FrameOwned::BulkString(Some(Bytes::from("foo"))),
        FrameOwned::BulkString(Some(Bytes::from("bar"))),
    ]))).await?;
    let resp = conn.read_frame().await?.unwrap();
    assert_eq!(format!("{:?}", resp), "SimpleString(\"OK\")");

    // Test GET
    conn.write_frame(&FrameOwned::Array(Some(vec![
        FrameOwned::BulkString(Some(Bytes::from("GET"))),
        FrameOwned::BulkString(Some(Bytes::from("foo"))),
    ]))).await?;
    let resp = conn.read_frame().await?.unwrap();
    assert_eq!(format!("{:?}", resp), "BulkString(Some(b\"bar\"))");

    // Test TTL
    conn.write_frame(&FrameOwned::Array(Some(vec![
        FrameOwned::BulkString(Some(Bytes::from("SET"))),
        FrameOwned::BulkString(Some(Bytes::from("ttl_key"))),
        FrameOwned::BulkString(Some(Bytes::from("ttl_val"))),
        FrameOwned::BulkString(Some(Bytes::from("EX"))),
        FrameOwned::BulkString(Some(Bytes::from("1"))),
    ]))).await?;
    let _ = conn.read_frame().await?.unwrap(); // OK

    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

    conn.write_frame(&FrameOwned::Array(Some(vec![
        FrameOwned::BulkString(Some(Bytes::from("GET"))),
        FrameOwned::BulkString(Some(Bytes::from("ttl_key"))),
    ]))).await?;
    let resp = conn.read_frame().await?.unwrap();
    assert_eq!(format!("{:?}", resp), "BulkString(None)");

    Ok(())
}
