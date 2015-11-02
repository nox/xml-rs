
use std::io;
use std::borrow::Cow;
use std::fmt;
use std::error;

use common::{Position, TextPosition};

use super::{Error, ErrorKind};

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.pos, self.msg())
    }
}

impl Position for Error {
    #[inline]
    fn position(&self) -> TextPosition { self.pos }
}

impl Error {
    // /// Creates a new error using position information from the provided
    // /// `Position` object and a message.
    // #[inline]
    // pub fn new<O: Position, S: Into<Cow<'static, str>>>(o: &O, msg: S) -> Error {
    //     Error { pos: o.position(), msg: msg.into() }
    // }

    /// Returns a reference to a message which is contained inside this error.
    #[inline]
    pub fn msg(&self) -> &str {
    	use super::ErrorKind::*;
    	match self.kind {
    		UnexpectedEof => &"Unexpected EOF",
    		Utf8( ref reason ) => error_description( reason ),
    		Io( ref io_error ) => error_description( io_error ),
    		Syntax( ref msg ) => msg.as_ref(),
    	}
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str { self.msg() }
}

impl<'a, P, M> From<( &'a P, M )> for Error where P: Position, M: Into<Cow<'static, str>> {
	fn from( orig: (&'a P, M) ) -> Self {
		Error{
			pos: orig.0.position(),
			kind: ErrorKind::Syntax( orig.1.into() )
		}
	}
}

impl From<::util::CharReadError> for Error {
	fn from( e: ::util::CharReadError ) -> Self {
		use ::util::CharReadError::*;
		Error{
			pos: TextPosition::new(),
			kind: match e {
				UnexpectedEof => ErrorKind::UnexpectedEof,
				Utf8( ref reason ) => ErrorKind::Utf8( reason.clone() ),
				Io( ref io_error ) =>
					ErrorKind::Io(
						io::Error::new(
							io_error.kind(),
							error_description( io_error )
						)
					),
			}
		}
	}
}

impl Clone for ErrorKind {
	fn clone( &self ) -> Self {
		use super::ErrorKind::*;
		match *self {
			UnexpectedEof => UnexpectedEof,
			Utf8( ref reason ) => Utf8( reason.clone() ),
			Io( ref io_error ) => Io( io::Error::new( io_error.kind(), error_description( io_error ) ) ),
			Syntax( ref msg ) => Syntax( msg.clone() ),
		}
	}
}
impl PartialEq for ErrorKind {
	fn eq( &self, other: &ErrorKind ) -> bool {
		use super::ErrorKind::*;
		match ( self, other ) {
			( &UnexpectedEof, &UnexpectedEof ) => true,
			( &Utf8( ref left ), &Utf8( ref right ) ) => left == right,
			( &Io( ref left ), &Io( ref right ) ) =>
				left.kind() == right.kind() &&
				error_description( left ) == error_description( right ),
			( &Syntax( ref left ), &Syntax( ref right ) ) =>
				left == right,

			( _, _ ) => false,
		}
	}
}
impl Eq for ErrorKind {}

fn error_description( e: &error::Error ) -> &str { e.description() }