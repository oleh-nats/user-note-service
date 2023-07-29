CREATE TABLE notes (
    id serial NOT NULL,
    title character varying(255) NOT NULL,
    content text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT notes_pkey PRIMARY KEY (id)
)