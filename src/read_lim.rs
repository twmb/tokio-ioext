use std::io;
use std::mem;

use futures::{Future, Poll};
use tokio_io::AsyncRead;

/// A future which can be used to read exactly `n` bytes into a buffer.
///
/// Created by the [`read_n`] function.
///
/// [`read_n`]: fn.read_n.html
#[derive(Debug)]
pub struct ReadN<A, T> {
    state: State<A, T>,
}

#[derive(Debug)]
enum State<A, T> {
    Reading { a: A, buf: T, pos: usize, n: usize },
    Empty,
}

/// Creates a future which will read exactly `n` bytes into `buf`.
///
/// The returned future will resolve to the I/O stream and the buffer once the read operation is
/// completed.
///
/// In the case of an error the buffer and the object will be discarded, with the error yielded.
pub fn read_n<A, T>(a: A, buf: T, n: usize) -> ReadN<A, T>
where
    A: AsyncRead,
    T: AsMut<[u8]>,
{
    ReadN {
        state: State::Reading { a, buf, pos: 0, n },
    }
}

impl<A, T> Future for ReadN<A, T>
where
    A: AsyncRead,
    T: AsMut<[u8]>,
{
    type Item = (A, T);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(A, T), io::Error> {
        match self.state {
            State::Reading {
                ref mut a,
                ref mut buf,
                ref mut pos,
                n,
            } => {
                let buf = buf.as_mut();
                while *pos < n {
                    let nn = try_nb!(a.read(&mut buf[*pos..n]));
                    *pos += nn;
                    if nn == 0 {
                        break;
                    }
                }
            }
            State::Empty => panic!("poll a ReadN after it's done"),
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { a, buf, .. } => Ok((a, buf).into()),
            State::Empty => panic!(),
        }
    }
}
