use async_graphql::connection::{self, Connection, CursorType, Edge, EmptyFields};
use async_graphql::SimpleObject;
use base64::{decode_config, encode_config, DecodeError, URL_SAFE_NO_PAD};
use std::convert::Infallible;
use std::fmt::Display;

#[derive(Debug)]
pub enum Base64CursorError {
    /// Invalid cursor. This can happen if the base64 string is valid, but its
    /// contents don't conform to the `name:index` pattern.
    Invalid,
    /// Decoding error. If this happens, the string isn't valid base64.
    DecodeError(DecodeError),
}

impl Display for Base64CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid cursor")
    }
}

/// Base64 cursor implementation
pub struct Base64Cursor {
    name: &'static str,
    index: usize,
}

impl Base64Cursor {
    /// Creates a new instance of the `Base64` cursor
    fn new(index: usize) -> Self {
        Self {
            name: "Cursor",
            index,
        }
    }

    /// Returns the `Cursor` encoded as Base64 string
    fn encode(&self) -> String {
        encode_config(format!("{}:{}", self.name, self.index), URL_SAFE_NO_PAD)
    }

    /// Decode the Base64 string representation of the `Cursor`
    fn decode(base64_str: &str) -> Result<Self, Base64CursorError> {
        let bytes =
            decode_config(base64_str, URL_SAFE_NO_PAD).map_err(Base64CursorError::DecodeError)?;

        let cursor = String::from_utf8(bytes).map_err(|_| Base64CursorError::Invalid)?;
        let index = cursor
            .split(':')
            .last()
            .map(|s| s.parse::<usize>())
            .ok_or(Base64CursorError::Invalid)?
            .map_err(|_| Base64CursorError::Invalid)?;

        Ok(Self::new(index))
    }

    /// Increments and return the index
    ///
    /// ## Overflow
    ///
    /// This function uses `saturating_add` to avoid overflow issues
    fn increment(&self) -> usize {
        self.index.saturating_add(1)
    }
}

impl From<Base64Cursor> for usize {
    fn from(cursor: Base64Cursor) -> Self {
        cursor.index
    }
}

impl CursorType for Base64Cursor {
    type Error = Base64CursorError;

    fn decode_cursor(s: &str) -> async_graphql::Result<Self, Self::Error> {
        Base64Cursor::decode(s)
    }

    fn encode_cursor(&self) -> String {
        self.encode()
    }
}

/// Additional fields for the connection instance
#[derive(SimpleObject)]
pub struct ConnectionFields {
    /// Total result set count
    total_count: usize,
}

/// Relay connection result
pub type ConnectionResult<T> =
    async_graphql::Result<Connection<Base64Cursor, T, ConnectionFields, EmptyFields>>;

/// Relay-compliant connection parameters
pub struct Params {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

impl Params {
    pub fn new(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Self {
        Self {
            after,
            before,
            first,
            last,
        }
    }
}

pub async fn query<T, I: ExactSizeIterator<Item = T>>(
    iter: I,
    params: Params,
    default_page_size: usize,
) -> ConnectionResult<T> {
    connection::query::<Base64Cursor, T, ConnectionFields, _, _, _, Infallible>(
        params.after,
        params.before,
        params.first,
        params.last,
        |after, before, first, last| async move {
            let iter_len = iter.len();
            let (start, end) = {
                let after: usize = after.map(|a| a.increment()).unwrap_or(0);
                let before: usize = before.map(|b| b.into()).unwrap_or(iter_len);

                match (first, last) {
                    (Some(first), _) => (after, (after.saturating_add(first)).min(before)),
                    (_, Some(last)) => ((before.saturating_sub(last)).max(after), before),
                    _ => (after, default_page_size.min(before)),
                }
            };

            let mut connection = Connection::with_additional_fields(
                start > 0,
                end < iter_len,
                ConnectionFields {
                    total_count: iter_len,
                },
            );

            connection.append(
                (start..end)
                    .into_iter()
                    .zip(iter.skip(start))
                    .map(|(cursor, node)| Edge::new(Base64Cursor::new(cursor), node)),
            );

            Ok(connection)
        },
    )
    .await
}
