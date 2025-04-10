# deser-stringified

sometimes, due to unfortunate layers of nesting, joining different systems, or whatever else, you get data as a string that's inside other, more structured data.

it'd be nice to be able to `serde` that data without doing the intermediate string handling.

**this is experimental**