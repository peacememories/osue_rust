use std::io::Write;
use std::io;

pub struct Compressor<W> {
    last: Option<(usize, u8)>,
    writer: W
}

impl <W: Write> Compressor<W> {
    pub fn new(writer: W) -> Self {
        Compressor {
            last: None,
            writer: writer
        }
    }

    fn write_comp(&mut self, amount: usize, byte: u8) -> io::Result<()> {
        try!(write!(self.writer, "{}", amount));
        self.writer.write_all(&[byte])
    }
}

impl <W: Write> Write for Compressor<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf {
            match self.last.take() {
                Some((amount, old_byte)) if byte == &old_byte => {
                    self.last = Some((amount+1, old_byte));
                },
                Some((amount, old_byte)) => {
                    self.last = Some((1, byte.clone()));
                    try!(self.write_comp(amount, old_byte));
                },
                None => {
                    self.last = Some((1, byte.clone()));
                }
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.last.take() {
            Some((amount, byte)) => {
                try!(self.write_comp(amount, byte));
            },
            _ => {}
        }
        self.writer.flush()
    }
}

pub struct Counter<T> {
    writer: T,
    written: usize
}

impl <T: Write> Counter<T> {
    pub fn new(writer: T) -> Self {
        Counter {
            writer: writer,
            written: 0
        }
    }

    pub fn written(&self) -> usize {
        self.written
    }
}

impl <T: Write> Write for Counter<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = try!(self.writer.write(buf));
        self.written += written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}