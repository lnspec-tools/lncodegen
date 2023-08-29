//! Compiler frontend that produce directly that
//! produce a list of tokens and then pass this list
//! of tokens to the parser. Then the parser will
//! produce the IR (intermediary representation) of
//! the source file.
//!
//! In our case, the language is really simple and
//! follow the following format
//!
//! ```ignore
//! Standard message types:
//! msgtype,<msgname>,<value>[,<option>]
//! msgdata,<msgname>,<fieldname>,<typename>,[<count>]
//!
//! TLV types:
//! tlvtype,<tlvstreamname>,<tlvname>,<value>
//! tlvdata,<tlvstreamname>,<tlvname>,<fieldname>,<typename>,[<count>]
//!
//! Subtypes:
//! subtype,<subtypename>
//! subtypedata,<subtypename>,<fieldname>,<typename>
//!
//! Note: <count> can be a fixed value, a named value read before,
//!     or '...' to read until the end of the current structure.
//! ```
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
pub mod parser;
pub mod scanner;
