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

CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    model VARCHAR NOT NULL DEFAULT 'gpt-3.5-turbo',
    prompt VARCHAR NOT NULL DEFAULT 'You are a helpful assistant.',
    kb VARCHAR NOT NULL DEFAULT '',
    certainty FLOAT NOT NULL DEFAULT 0.9,
    max_tokens INT NOT NULL DEFAULT 512,
    max_length INT NOT NULL DEFAULT 512,
    top_p FLOAT NOT NULL DEFAULT 1.0,
    temperature FLOAT NOT NULL DEFAULT 0.5,
    frequency_penalty FLOAT NOT NULL DEFAULT 0.0,
    presence_penalty FLOAT NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS replies (
    id SERIAL PRIMARY KEY,
    look_for VARCHAR NOT NULL,
    reply_with VARCHAR NOT NULL,
    expiration TIMESTAMP DEFAULT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS scheduled_messages (
    id SERIAL PRIMARY KEY,
    channel VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    post_at TIMESTAMP DEFAULT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);