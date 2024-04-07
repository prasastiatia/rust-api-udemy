-- Your SQL goes here
CREATE TABLE nilaiSiswa (
    id SERIAL PRIMARY KEY,
    dataSiswa_id INTEGER NOT NULL REFERENCES dataSiswa(id),
    nilai VARCHAR(64) NOT NULL, 
    name VARCHAR (128) NOT NULL,
    mataPelajaran VARCHAR(64) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)