extern crate iron;
extern crate router;
extern crate time;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(_: &mut Request) -> IronResult<Response> {
        let now = time::now();

        let midnight = set_hour(now, 0);
        let midday = set_hour(now, 12);
        let sunset = set_hour(now, 18);
        let midnightnextday = set_hour(now, 24);

        let greeting = if now >= midnight && now < midday {
            "Good Morning".to_string()
        } else if now >= midday && now < sunset {
            "Good Afternoon".to_string()
        } else if now >= sunset && now < midnightnextday {
            "Good Evening".to_string()
        } else {
            "Hello".to_string()
        };

        Ok(Response::with((status::Ok, greeting)))
    }
}

fn set_hour(t: time::Tm, hour: i32) -> time::Tm {
    let mut t = t;
    t.tm_hour = hour;
    t.tm_min = 0;
    t.tm_sec = 0;
    t.tm_nsec = 0;
    t
}
