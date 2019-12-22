use std::{
    cell::RefCell,
    io::{self, Read, Write},
    iter::Iterator,
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct Stream {
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl Stream {
    pub fn new() -> Self {
        Self {
            buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self.buffer.try_borrow_mut() {
            Ok(mut borrowed) => {
                borrowed.extend_from_slice(buf);
                Ok(buf.len())
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Buffer was already in use",
            )),
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match self.buffer.try_borrow_mut() {
            Ok(mut borrowed) => {
                let mut counter = 0;
                let mut buf_ptr = buf.iter_mut();
                for byte in borrowed.iter() {
                    if let Some(value) = buf_ptr.next() {
                        *value = *byte;
                    } else {
                        break;
                    }
                    counter += 1;
                }
                borrowed.drain(0..counter);

                Ok(counter)
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Buffer was already in use",
            )),
        }
    }
}
