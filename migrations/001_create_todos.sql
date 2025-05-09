CREATE TABLE todos (
                       id UUID PRIMARY KEY,
                       title TEXT NOT NULL,
                       completed BOOLEAN NOT NULL DEFAULT false,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
