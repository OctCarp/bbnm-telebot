use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct VideoInfo {
    title: String,
    desc: String,
    owner: Owner,
    top_reply: Option<Reply>,
}

#[derive(Debug, Deserialize)]
struct Owner {
    mid: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Reply {
    member: Owner,
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Content {
    message: String,
}

