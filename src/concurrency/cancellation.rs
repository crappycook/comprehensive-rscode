#![allow(unused)]
use std::io::{self, ErrorKind};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};

struct LinesReader {
    stream: DuplexStream,
    bytes: Vec<u8>,
    buf: [u8; 1],
}

impl LinesReader {
    fn new(stream: DuplexStream) -> Self {
        Self {
            stream,
            bytes: Vec::new(),
            buf: [0],
        }
    }

    async fn next(&mut self) -> io::Result<Option<String>> {
        // The local variables lose parts of the string
        // let mut bytes = Vec::new();
        // let mut buf = [0];
        while self.stream.read(&mut self.buf[..]).await? != 0 {
            self.bytes.push(self.buf[0]);
            if self.buf[0] == b'\n' {
                break;
            }
        }
        if self.bytes.is_empty() {
            return Ok(None);
        }
        // Alternative: use `self.bytes.clear()` to free memory
        // let s = self.bytes.clone();
        // self.bytes.clear();
        let res = std::mem::take(&mut self.bytes);
        let s = String::from_utf8(res)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "not UTF-8"))?;
        Ok(Some(s))
    }
}

async fn slow_copy(source: String, mut dest: DuplexStream) -> std::io::Result<()> {
    for b in source.bytes() {
        dest.write_u8(b).await?;
        tokio::time::sleep(Duration::from_millis(10)).await
    }
    Ok(())
}

#[tokio::main]
async fn run_copy() -> std::io::Result<()> {
    let (client, server) = tokio::io::duplex(5);
    let source = String::from("read\nAPI\ndocumentation\n");
    let handle = tokio::spawn(slow_copy(source, client));

    let mut lines = LinesReader::new(server);
    let mut interval = tokio::time::interval(Duration::from_millis(60));
    loop {
        tokio::select! {
            // Whenever the `tick()` branch finishes first, `next()` and its `buf` are dropped.
            _ = interval.tick() => println!("tick!"),
            line = lines.next() => if let Some(l) = line? {
                print!("{}", l)
            } else {
                break
            },
        }
    }
    handle.await.unwrap()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancellation() {
        run_copy();
    }
}
