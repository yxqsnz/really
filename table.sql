CREATE TABLE IF NOT EXISTS asset (hash TEXT PRIMARY KEY, name TEXT NOT NULL,
                                  provider TEXT NOT NULL, nsfw BOOL NOT NULL, category TEXT NOT NULL, content BLOB NOT NULL);

