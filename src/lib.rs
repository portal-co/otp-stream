#![no_std]
#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
use core::convert::Infallible;
use core::marker::PhantomData;

use alloc::vec;
use alloc::vec::Vec;
use sha3::digest::XofReader;
#[repr(transparent)]
#[derive(Clone,Copy,Debug,Default)]
pub struct Digest<X,E>{
    pub wrapped: X,
    pub errors: PhantomData<E>,
}
impl<X: XofReader,E: embedded_io::Error> embedded_io::ErrorType for Digest<X,E>{
    type Error = E;
}
impl<X: XofReader,E: embedded_io::Error> embedded_io::Read for Digest<X,E>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.wrapped.read(buf);
        Ok(buf.len())
    }
}
pub struct Otp<A,B>{
    pub wrapped: A,
    pub pad: B,
}
#[cfg(feature = "std")]
impl<A: std::io::Read,B: embedded_io::Read> std::io::Read for Otp<A,B> where std::io::Error: From<B::Error>{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let x = self.wrapped.read(buf)?;
        let mut b = vec![0u8; x];
        let y = self.pad.read(&mut b)?;
        for (a,b) in b.iter().zip(buf.iter_mut()){
            *b ^= *a;
        }
        return Ok(y);
    }
}
#[cfg(feature = "std")]
impl<A: std::io::Write,B: embedded_io::Read> std::io::Write for Otp<A,B> where std::io::Error: From<B::Error>{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut n = vec![0u8; buf.len()];
        let y = self.pad.read(&mut n)?;
        let buf: Vec<_> = buf[..y].iter().zip(n.iter()).map(|(a,b)|*a ^ *b).collect();
        return self.wrapped.write(&buf);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.wrapped.flush()
    }
}

impl<A: embedded_io_async::Read,B: embedded_io::Read> embedded_io_async::Read for Otp<A,B> where A::Error: From<B::Error>{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let x = self.wrapped.read(buf).await?;
        let mut b = vec![0u8; x];
        let y = self.pad.read(&mut b)?;
        for (a,b) in b.iter().zip(buf.iter_mut()){
            *b ^= *a;
        }
        return Ok(y);
    }
}
impl<A: embedded_io_async::Write,B: embedded_io::Read> embedded_io_async::Write for Otp<A,B>  where A::Error: From<B::Error>{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut n = vec![0u8; buf.len()];
        let y = self.pad.read(&mut n)?;
        let buf: Vec<_> = buf[..y].iter().zip(n.iter()).map(|(a,b)|*a ^ *b).collect();
        return self.wrapped.write(&buf).await;
    }
}
impl<A: embedded_io::ErrorType,B: embedded_io::Read> embedded_io::ErrorType for Otp<A,B>{
    type Error = A::Error;
}
impl<A: embedded_io::Read,B: embedded_io::Read> embedded_io::Read for Otp<A,B> where A::Error: From<B::Error>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let x = self.wrapped.read(buf)?;
        let mut b = vec![0u8; x];
        let y = self.pad.read(&mut b)?;
        for (a,b) in b.iter().zip(buf.iter_mut()){
            *b ^= *a;
        }
        return Ok(y);
    }
}
impl<A: embedded_io::Write,B: embedded_io::Read> embedded_io::Write for Otp<A,B>  where A::Error: From<B::Error>{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut n = vec![0u8; buf.len()];
        let y = self.pad.read(&mut n)?;
        let buf: Vec<_> = buf[..y].iter().zip(n.iter()).map(|(a,b)|*a ^ *b).collect();
        return self.wrapped.write(&buf);
    }
    
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wrapped.flush()
    }
}
