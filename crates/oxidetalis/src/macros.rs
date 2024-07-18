// OxideTalis Messaging Protocol homeserver implementation
// Copyright (C) 2024 OxideTalis Developers <otmp@4rs.nl>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://gnu.org/licenses/agpl-3.0>.

//! OxideTalis server macros, to make the code more readable and easier to
//! write.

/// Macro to return a [`ServerEvent`] with a [`WsError::InternalServerError`] if
/// the result of an expression is an [`Err`].
///
/// ## Example
/// ```rust,ignore
/// fn example() -> ServerEvent {
///    // some_function() returns a Result, if it's an Err, return an
///    // ServerEvent::InternalServerError
///    let result = try_ws!(some_function());
///    ServerEvent::from(result)
/// }
/// ```
///
/// [`ServerEvent`]: crate::websocket::ServerEvent
/// [`WsError::InternalServerError`]: crate::websocket::errors::WsError::InternalServerError
/// [`Err`]: std::result::Result::Err
#[macro_export]
macro_rules! try_ws {
    (Some $result_expr:expr) => {
        match $result_expr {
            Ok(val) => val,
            Err(err) => {
                log::error!("{err}");
                return Some(
                    $crate::websocket::ServerEvent::<$crate::websocket::Unsigned>::from(
                        $crate::websocket::errors::WsError::from(err),
                    ),
                );
            }
        }
    };
}

/// Macro to create the `WsError` enum with the given error names and reasons.
///
/// ## Example
/// ```rust,ignore
/// ws_errors! {
///    FirstError = "This is the first error",
///    SecondError = "This is the second error",
/// }
/// ```
#[macro_export]
macro_rules! ws_errors {
    ($($name:ident = $reason:tt),+ $(,)?) => {
        #[derive(Debug, thiserror::Error)]
        #[doc = "Websocket errors, returned in the websocket communication"]
        pub enum WsError {
            $(
                #[doc = $reason]
                #[error($reason)]
                $name
            ),+
        }
        impl WsError {
            #[doc = "Returns error name"]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(
                        WsError::$name => stringify!($name)
                    ),+
                }
            }
            #[doc = "Returns the error reason"]
            pub const fn reason(&self) -> &'static str {
                match self {
                    $(
                        WsError::$name => $reason
                    ),+
                }
            }
        }
    };
}
