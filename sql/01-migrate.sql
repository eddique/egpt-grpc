CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    role VARCHAR NOT NULL,
    active BOOL NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
CREATE TABLE IF NOT EXISTS chats (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS messages (
    id SERIAL PRIMARY KEY,
    text VARCHAR NOT NULL,
    user_name VARCHAR NOT NULL,
    chat_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS message_metadata (
    id SERIAL PRIMARY KEY,
    message_id INTEGER NOT NULL,
    model VARCHAR NOT NULL,
    temperature FLOAT NOT NULL,
    max_tokens INTEGER NOT NULL,
    certainty FLOAT NOT NULL,
    kb_answer BOOL NOT NULL,
    article_title VARCHAR NOT NULL,
    article_link VARCHAR NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
);