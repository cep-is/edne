//
// Copyright (c) 2025 murilo ijanc' <murilo@ijanc.org>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use std::collections::HashMap;

use crate::{
    error::Error,
    model::{Localities, Locality, LocalityId},
};

/// Parses localities from a `BufRead` stream.
/// Expected line layout: `id@name` (ISO-8859-1; split by `@`).
pub fn parse<R: std::io::BufRead>(reader: R) -> Result<Localities, Error> {
    parse_capacity(reader, 0)
}

/// Same as [`parse_localities`], but lets you hint the expected size to
/// reduce rehashing.
pub fn parse_capacity<R: std::io::BufRead>(
    mut reader: R,
    capacity_hint: usize,
) -> Result<Localities, Error> {
    let mut raw = Vec::<u8>::with_capacity(8 * 1024);
    let mut text = String::with_capacity(8 * 1024);
    let mut map = if capacity_hint > 0 {
        HashMap::with_capacity(capacity_hint)
    } else {
        HashMap::new()
    };

    let mut line_no: u64 = 0;

    loop {
        raw.clear();
        let n = reader.read_until(b'\n', &mut raw)?;
        if n == 0 {
            break;
        }
        line_no += 1;

        while matches!(raw.last(), Some(b'\n' | b'\r')) {
            raw.pop();
        }
        if raw.is_empty() {
            continue;
        }

        // ISO-8859-1 → UTF-8 (Latin-1 is 1:1 to Unicode U+00xx)
        text.clear();
        text.reserve(raw.len());
        for &b in &raw {
            text.push(char::from(b));
        }

        let mut it = text.split('@');

        let id_str = match it.next() {
            Some(v) => v,
            None => {
                return Err(Error::FieldCount {
                    line: line_no,
                    got: 1,
                    expected: 2,
                });
            }
        };

        it.next(); // sigla da uf

        let name_str = match it.next() {
            Some(v) => v,
            None => {
                return Err(Error::FieldCount {
                    line: line_no,
                    got: 1,
                    expected: 2,
                });
            }
        };

        let id = match id_str.trim().parse::<u32>() {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::ParseInt { line: line_no, source: e });
            }
        };

        let name = name_str.trim().to_string();

        let nh = Locality { id: LocalityId::new(id), name };
        map.insert(nh.id, nh);
    }

    Ok(Localities::new(map))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn parse_localities_basic() {
        assert!(true);
        let data = b"15321@AC@Terra Ind\xEDgena Mamoadate@69939810@0@P@2@Terra Ind Mamoadate@\n
        13@AC@Pl\xE1cido de Castro@69928000@0@M@@Pl\xE1cido Castro@1200385\n";
        let cur = Cursor::new(&data[..]);

        let nhs = parse(cur).expect("parse ok");
        assert_eq!(nhs.len(), 2);

        let n1 = nhs.get(LocalityId::new(15321)).unwrap();
        assert_eq!(n1.name, "Terra Indígena Mamoadate");

        let n2 = nhs.get(LocalityId::new(13)).unwrap();
        assert_eq!(n2.name, "Plácido de Castro");
    }

    #[test]
    fn error_invalid_id() {
        let data = b"X@AC@Terra Ind\xEDgena Mamoadate@69939810@0@P@2@Terra Ind Mamoadate@\n";
        let cur = Cursor::new(&data[..]);

        let err = parse(cur).unwrap_err();
        match err {
            Error::ParseInt { line, .. } => {
                assert_eq!(line, 1);
            }
            _ => panic!("expected ParseInt error"),
        }
    }

    #[test]
    fn error_missing_separator() {
        let data = b"X|AC|Terra Ind\xEDgena Mamoadate|69939810|0|P|2|Terra Ind Mamoadate|\n";
        let cur = Cursor::new(&data[..]);

        let err = parse(cur).unwrap_err();
        match err {
            Error::FieldCount { line, got, expected } => {
                assert_eq!(line, 1);
                assert_eq!(got, 1);
                assert_eq!(expected, 2);
            }
            _ => panic!("expected FieldCount error"),
        }
    }
}
