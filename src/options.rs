extern crate getopts;

use getopts::Options;
use getopts::Matches;

use riemann::*;

pub fn get_options() -> Options {
    let mut opts = Options::new();

    opts.optflag( "h", "help",      "Print this message");
    opts.optopt(  "",  "riemann-host", "Host of the Riemann server to write to", "localhost");
    opts.optopt(  "",  "riemann-port", "Port of the Riemann server to write to", "5555");
    opts.optopt(  "",  "time",      "Unix timestamp of the event", "");
    opts.optopt(  "",  "state",     "Service state", "");
    opts.optopt(  "",  "host",      "Hostname the event originates from", "");
    opts.optopt(  "d", "description", "Description of the event", "");
    opts.optmulti("t", "tag",       "Append a tag to the event. Can be specified multiple times", "");
    opts.optopt(  "",  "ttl",       "Event Time To Live", "");
    opts.optmulti("a", "attribute", "Append an attribute to the event. Can be specified multiple times", "key=value");
    opts.optflag(  "",  "int64",     "Send metric as a signed 64 bit integer value");
    opts.optflag(  "",  "double",    "Send metric as a 64 bit double value");
    opts.optflag(  "",  "float",     "Send metric as a 32 bit float value");

    return opts;
}

#[derive(Debug)]
pub struct RiemannServer {
    host: Option<String>,
    port: Option<i32>
}

impl RiemannServer {
    pub fn from_args(matches: &Matches) -> Result<RiemannServer, String> {
        let server = RiemannServer {
            host: matches.opt_str("riemann-host"),
            port: match matches.opt_str("riemann-port") {
                Some(ps) => match ps.parse::<i32>() {
                    Ok(p) => Some(p),
                    Err(_) => return Err("--riemann-port must be an integer".to_string())
                },
                None => None
            }
        };

        return Ok(server);
    }
}

pub enum Metric {
    Int(i64),
    Double(f64),
    Float(f32)
}

pub fn parse_metric(value: &String, matches: &Matches) -> Result<Metric, String> {
    let mut result: Option<Metric> = None;

    if matches.opt_present("int64") {
        result = match value.parse::<i64>() {
            Ok(i)  => { Some(Metric::Int(i)) },
            Err(_) => return Err("Could not parse metric as a signed integer".to_string())
        }
    }

    if matches.opt_present("float") {
        if result.is_none() {
            result = match value.parse::<f32>() {
                Ok(f)  => { Some(Metric::Float(f)) },
                Err(_) => return Err("Could not parse metric as a float".to_string())
            }
        } else {
            return Err("You can only set one of --int64, --double, --float".to_string());
        }
    }

    if matches.opt_present("double") {
        if result.is_none() {
            result = match value.parse::<f64>() {
                Ok(d)  => { Some(Metric::Double(d)) },
                Err(_) => return Err("Could not parse metric as a double".to_string())
            }
        } else {
            return Err("You can only set one of --int64, --double, --float".to_string());
        }
    }

    // Explict type not provided, try to guess
    if result.is_none() {
        result = match value.parse::<i64>() {
            Ok(i) => { Some(Metric::Int(i)) },
            Err(_) => match value.parse::<f64>() {
                Ok(d) => { Some(Metric::Double(d)) },
                Err(_) => return Err("Could not determine metric type".to_string())
            }
        }
    }

    if let Some(m) = result {
        return Ok(m);
    } else {
        return Err("Unable to parse metric value for unknown reason".to_string());
    }
}

pub fn marshall(matches: &Matches) -> Result<Event,String> {
    return marshall_onto(matches, Event::new());
}

pub fn marshall_onto(matches: &Matches, event: Event) -> Result<Event,String> {
    let mut event = event.clone();

    if let Some(t) = matches.opt_str("time") {
        match t.parse::<i64>() {
            Ok(i) => event.set_time(i),
            Err(_) => return Err("The value for --time must be an integer".to_string())
        }
    }

    if let Some(s) = matches.opt_str("state") {
        event.set_state(s);
    }

    if let Some(h) = matches.opt_str("host") {
        event.set_host(h);
    } else {
        event.set_host("dummy hostname...".to_string());
    }

    if let Some(d) = matches.opt_str("description") {
        event.set_description(d);
    }

    for s in matches.opt_strs("tag") {
        event.mut_tags().push(s);
    }

    if let Some(t) = matches.opt_str("ttl") {
        match t.parse::<f32>() {
            Ok(f) => event.set_ttl(f),
            Err(_) => return Err("Value for --ttl must be a number".to_string())
        }
    }

    if !matches.free.is_empty() {
        event.set_service(matches.free[0].clone());

        if matches.free.len() == 2 {
            match parse_metric(&matches.free[1], &matches) {
                Ok(r) => match r {
                    Metric::Int(i)    => event.set_metric_sint64(i),
                    Metric::Double(d) => event.set_metric_d(d),
                    Metric::Float(f)  => event.set_metric_f(f)
                },

                Err(e) => return Err(e)
            }
        }
    }

    return Ok(event);
}
