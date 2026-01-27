CREATE TABLE
       IF NOT EXISTS tickets (
              id TEXT PRIMARY KEY,
              title TEXT NOT NULL,
              description TEXT,
              priority TEXT NOT NULL,
              status TEXT NOT NULL,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
       );