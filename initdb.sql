CREATE DATABASE banco_dos_amigos;

\connect banco_dos_amigos;

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS pessoas (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    nome VARCHAR(255) NOT NULL,
    idade INTEGER NOT NULL,
    altura DECIMAL(3,2) NOT NULL,
    peso DECIMAL(5,2) NOT NULL
);

-- drop table pessoas;

-- ALTER TABLE pessoas 
-- ALTER peso TYPE DECIMAL(5,2);