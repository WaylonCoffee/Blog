use chrono::LocalTime;

struct Post {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub created_at: DateTime,
    pub content: String,
    pub published: bool,
}

impl Post {
    fn new(title: String, description: String, content: String) -> Self {
        Post {
            title: title,
            description: description,
            content: content,
            created_at: Local::now(),
            published: false,
        }
    }
}
