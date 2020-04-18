# Hello

Hello is a web server that implements bare bones thread pool to handle requests asynchronously.

Right now, the root will return a hello webpage, `/sleep` -- a hello page after some time, and any other URI -- a 404 page.

### How to run
Just one line

```
$ cargo run --release
```