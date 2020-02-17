use crate::Compat;
use futures_core::{
    task::{self, Poll},
};
use std::pin::Pin;
use std::io;

impl<T> tokio::prelude::AsyncRead for Compat<T>
where
    T: futures_io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        futures_io::AsyncRead::poll_read(self.project().inner, cx, buf)
    }
}

impl<T> futures_io::AsyncRead for Compat<T>
where
    T: tokio::prelude::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        tokio::prelude::AsyncRead::poll_read(self.project().inner, cx, buf)
    }
}

impl<T> tokio::prelude::AsyncBufRead for Compat<T>
where
    T: futures_io::AsyncBufRead,
{
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut task::Context,
    ) -> Poll<io::Result<&'a [u8]>> {
        futures_io::AsyncBufRead::poll_fill_buf(self.project().inner, cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        futures_io::AsyncBufRead::consume(self.project().inner, amt)
    }
}

impl<T> futures_io::AsyncBufRead for Compat<T>
where
    T: tokio::prelude::AsyncBufRead,
{
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut task::Context,
    ) -> Poll<io::Result<&'a [u8]>> {
        tokio::prelude::AsyncBufRead::poll_fill_buf(self.project().inner, cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        tokio::prelude::AsyncBufRead::consume(self.project().inner, amt)
    }
}

impl<T> tokio::prelude::AsyncWrite for Compat<T>
where
    T: futures_io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        futures_io::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        futures_io::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        futures_io::AsyncWrite::poll_close(self.project().inner, cx)
    }
}

impl<T> futures_io::AsyncWrite for Compat<T>
where
    T: tokio::prelude::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        tokio::prelude::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        tokio::prelude::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        tokio::prelude::AsyncWrite::poll_shutdown(self.project().inner, cx)
    }
}
