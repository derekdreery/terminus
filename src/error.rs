//! Possible errors defined using the `error-chain` crate

use super::Capability;

error_chain! {

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        NotSupported(c: Capability) {
            description("the terminal does not have the required capability"),
            display("the terminal does not have the \"{}\" capability", c)
        }
    }
}