extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::Client;
use std::io::{self, Write};
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    // https appears not to work
    let uri = "http://httpbin.org/get".parse().unwrap(); // works

    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body()
            .for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
    });
    core.run(work).unwrap();
}

// Response: 200 OK
// {
//   "args": {},
//   "headers": {
//     "Host": "httpbin.org",
//     "X-Amzn-Trace-Id": "Root=1-5fe95ac3-3cba9c8064c11382772cc264"
//   },
//   "origin": "37.120.149.92",
//   "url": "http://httpbin.org/get"
// }
