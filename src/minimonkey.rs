use std::convert::TryInto;
use std::i16;
use std::io::Error;
use std::io::{Read, Write};
use std::str;

pub struct Response {
    code: u8,
    data: Vec<u8>,
}

impl Response {
    pub fn new(code: u8, data: Vec<u8>) -> Self {
        Response { code, data }
    }

    pub fn print(&self) {
        if self.code == 0x7f {
            let msg = str::from_utf8(&self.data).unwrap();
            println!("[debug] {:?}", msg);
            return;
        }
        if self.code == 0x7e {
            let msg = str::from_utf8(&self.data).unwrap();
            println!("[error] {:?}", msg);
            return;
        }
        if self.code == 0x02 {
            let msg = str::from_utf8(&self.data).unwrap();
            println!("[room] {:?}", msg);
            return;
        }
        if self.code == 0x03 {
            let msg = str::from_utf8(&self.data).unwrap();
            println!("[msg] {:?}", msg);
            return;
        }
        println!("Unhandle code: {:?}", self.code);
    }

    pub fn as_string(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

fn common(stream: &mut impl Write, code: &[u8], payload: &[u8]) -> Result<usize, Error> {
    let size = payload.len();
    let size: [u8; 8] = size.to_le_bytes();

    let mut written: usize = 0;

    written += stream.write(&code)?;
    written += stream.write(&size[0..2])?;
    written += stream.write(&payload)?;

    Ok(written)
}

pub fn read(stream: &mut impl Read) -> Result<Response, Error> {
    let mut code: [u8; 1] = [0x00];
    let mut size: [u8; 2] = [0x00, 0x00];

    stream.read(&mut code)?;
    stream.read(&mut size)?;

    let size = i16::from_le_bytes(size);
    let size: usize = size.try_into().unwrap();

    let mut buf = Vec::with_capacity(size);
    stream
        .take(size.try_into().unwrap())
        .read_to_end(&mut buf)?;

    Ok(Response::new(code[0], buf))
}

pub fn authenticate(stream: &mut impl Write, token: &str) -> Result<usize, Error> {
    common(stream, &[0x01], token.as_bytes())
}

pub fn enter(stream: &mut impl Write, room: &str) -> Result<usize, Error> {
    common(stream, &[0x02], room.as_bytes())
}

pub fn publish(stream: &mut impl Write, data: &[u8]) -> Result<usize, Error> {
    common(stream, &[0x03], data)
}

pub fn subscribe(stream: &mut impl Write, tag: &str) -> Result<usize, Error> {
    common(stream, &[0x04], tag.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::BufWriter;

    #[test]
    fn test_to_authenticate() {
        let mut dir = env::temp_dir();
        dir.push("test_to_authenticate.bin");
        let file = File::create(dir).unwrap();
        let mut file = BufWriter::new(&file);
        let written = authenticate(&mut file, "token").unwrap();
        assert_eq!(written, 1 + 2 + 5);
    }
}
