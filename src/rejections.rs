use warp::reject::Reject;
use std::fmt;

// Define a custom rejection for invalid headers or parameters
#[derive(Debug)]
pub struct InvalidParameter {
    pub message: String,
}

impl Reject for InvalidParameter {}

impl fmt::Display for InvalidParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct NotFound {
    pub message: String,
}

impl Reject for NotFound {}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
