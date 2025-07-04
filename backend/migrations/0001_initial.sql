CREATE TABLE IF NOT EXISTS items (
                                     id TEXT PRIMARY KEY,
                                     name TEXT NOT NULL,
                                     price REAL NOT NULL,
                                     description TEXT
);

CREATE TABLE IF NOT EXISTS orders (
                                      id TEXT PRIMARY KEY,
                                      item_id TEXT NOT NULL,
                                      contact_info TEXT NOT NULL,
                                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                      status TEXT DEFAULT 'new'
);