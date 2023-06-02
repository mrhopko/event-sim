mod server;
use bytes::BytesMut;
use tokio::io::BufWriter;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use server::event_bus::EventBus;
use server::message::{Request, RequestType};

struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_frame(&mut self) -> Result<()> {}
}

#[tokio::main]
async fn main() {
    // listen for messages and convert from tcp to frames.
    // spawn a request to manage conversion and then send to bus manager

    //    let listener = TcpListener::bind("127.0.0.1:6379");
    let mut bus = EventBus::new();

    // channel used to talk to the database
    let (manager_tx, mut manager_rx) = mpsc::channel(32);
    let manager_tx1 = manager_tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let request = Request {
            request_type: RequestType::Count,
            message: "".into(),
            event: None,
            responder: resp_tx,
        };
        // send to the manager
        manager_tx1.send(request).await.unwrap();
        // await response on responder channel
        let resp = resp_rx.await;
        println!("GOT: {:?}", resp);
    });

    // task that manages the database
    // does not interact with clients directly
    let bus_manager = tokio::spawn(async move {
        while let Some(request) = manager_rx.recv().await {
            let response = bus.request(&request);
            request.responder.send(response).unwrap();
        }
    });

    t1.await.unwrap();
    bus_manager.await.unwrap();
}
