use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use std::time::Instant;

#[derive(Copy, Clone)]
struct RequestTimer(Option<Instant>);

pub struct TimeRequests;
impl Fairing for TimeRequests {
    fn info(&self) -> Info {
        Info {
            name: "Time Requests",
            kind: Kind::Request | Kind::Response,
        }
    }
    fn on_request(&self, req: &mut Request, _: &Data) {
        // Store request processing start time
        req.local_cache(|| RequestTimer(Some(Instant::now())));
    }

    fn on_response(&self, req: &Request, _response: &mut Response) {
        match req.local_cache(|| RequestTimer(None)).0 {
            Some(t1) => {
                let dt = t1.elapsed();
                let dt_string = format!("{}.{}ms", dt.as_millis(), dt.as_micros());
                println!("Request to {} took: {}", req.uri().path(), dt_string)
            }
            None => (),
        }
    }
}
