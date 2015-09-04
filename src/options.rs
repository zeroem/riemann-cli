extern crate getopts;

use getopts::Options;
use getopts::Matches;

use riemann::*;

pub fn get_options() -> Options {
    let mut opts = Options::new();

    opts.optflag( "h", "help",      "Print this message");
    opts.optopt(  "",  "time",      "Unix timestamp of the event", "");
    opts.optopt(  "",  "state",     "Service state", "");
    opts.optopt(  "",  "host",      "Hostname the event originates from", "");
    opts.optopt(  "d", "description", "Description of the event", "");
    opts.optmulti("t", "tag",       "Append a tag to the event. Can be specified multiple times", "");
    opts.optopt(  "",  "ttl",       "Event Time To Live", "");
    opts.optmulti("a", "attribute", "Append an attribute to the event. Can be specified multiple times", "key=value");
    opts.optopt(  "",  "int64",     "Event metric as a signed 64 bit integer value", "42");
    opts.optopt(  "",  "double",    "Event metric as a 64 bit double value", "3.141");
    opts.optopt(  "",  "float",     "Event metric as a 32 bit float value", "3.141");

    return opts;
}

pub fn marshall(matches: Matches) -> Result<Event,String> {
    return marshall_onto(matches, Event::new());
}

pub fn marshall_onto(matches: Matches, event: Event) -> Result<Event,String> {
    let mut event = event.clone();
    if matches.opt_present("time") {
        event.set_time(matches.opt_str("time").unwrap().parse::<i64>().unwrap());
    }
    return Ok(event);

}
