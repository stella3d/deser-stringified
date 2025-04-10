# deser-stringified

sometimes, due to unfortunate layers of nesting, joining different systems, or whatever else, you get data as a string that's inside other, more structured data.

it'd be nice to be able to `serde` that data without doing the intermediate string handling.

**this is experimental**


## special thanks

to this bluesky poster who responded to my tweet asking about a better way of deserializing stringified data:

<blockquote class="bluesky-embed" data-bluesky-uri="at://did:plc:awpmnhm4q4y62hwxukiwg6ry/app.bsky.feed.post/3lmidya2lvc2m" data-bluesky-cid="bafyreigwsilgj2zlsgju2or2b5ppteac3bhwkd2kiflcssfce3tkq6l5gi" data-bluesky-embed-color-mode="system"><p lang="en">hmmm what if you write a custom deserializer with a visit_str that calls into serde_json?

play.rust-lang.org?version=stab...</p>&mdash; ari :3 🏳️‍⚧️ (<a href="https://bsky.app/profile/did:plc:awpmnhm4q4y62hwxukiwg6ry?ref_src=embed">@ari.gf</a>) <a href="https://bsky.app/profile/did:plc:awpmnhm4q4y62hwxukiwg6ry/post/3lmidya2lvc2m?ref_src=embed">April 10, 2025 at 3:32 PM</a></blockquote>

i've used a more generic form of that approach in this crate.