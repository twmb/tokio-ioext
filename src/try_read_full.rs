use std::io;
use std::mem;

use futures::{Future, Poll};
use tokio_io::AsyncRead;

/// A future which can be used to read as many bytes as possible into a buffer.
///
/// Created by the [`try_read_full`] function.
///
/// [`try_read_full`]: fn.try_read_full.html
#[derive(Debug)]
pub struct TryReadFull<A, T> {
    state: State<A, T>,
}

#[derive(Debug)]
enum State<A, T> {
    Reading { a: A, buf: T, pos: usize },
    Empty,
}

/// Creates a future which will read as many bytes as possible into `buf` and return the number of
/// bytes read.
///
/// The returned future will resolve to the I/O stream, the buffer, and the total bytes read once
/// the read operation is completed.
///
/// In the case of an error the buffer and the object will be discarded, with the error yielded.
pub fn try_read_full<A, T>(a: A, buf: T) -> TryReadFull<A, T>
where
    A: AsyncRead,
    T: AsMut<[u8]>,
{
    TryReadFull {
        state: State::Reading { a, buf, pos: 0 },
    }
}

impl<A, T> Future for TryReadFull<A, T>
where
    A: AsyncRead,
    T: AsMut<[u8]>,
{
    type Item = (A, T, usize);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(A, T, usize), io::Error> {
        match self.state {
            State::Reading {
                ref mut a,
                ref mut buf,
                ref mut pos,
            } => {
                let buf = buf.as_mut();
                while *pos < buf.len() {
                    let n = try_nb!(a.read(&mut buf[*pos..]));
                    *pos += n;
                    if n == 0 {
                        break;
                    }
                }
            }
            State::Empty => panic!("poll a TryReadFull after it's done"),
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { a, buf, pos } => Ok((a, buf, pos).into()),
            State::Empty => panic!(),
        }
    }
}
