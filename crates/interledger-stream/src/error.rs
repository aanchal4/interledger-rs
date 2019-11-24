use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Error connecting: {}", _0)]
    ConnectionError(String),
    #[fail(display = "Error polling: {}", _0)]
    PollError(String),
    #[fail(display = "Error polling: {}", _0)]
    SendMoneyError(String),
<<<<<<< HEAD
    #[fail(display = "Too many rejected packets error: {}", _0)]
=======
    #[fail(display = "Error connecting: {}", _0)]
>>>>>>> 05125e390939c82460013ab188a2a7a310f4d6cc
    TooManyRejectedPacketsError(String),
}
