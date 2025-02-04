CREATE TABLE IF NOT EXISTS users (
    user_id SMALLSERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password BYTEA NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS clothes (
    clothe_id SMALLSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    color SMALLINT NOT NULL,
    category SMALLINT NOT NULL,
    user_id SMALLINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    is_for_hot_weather BOOLEAN DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS outfits (
    outfit_id SMALLSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id SMALLINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    -- Bitmask to store multiple colors
    color_mask SMALLINT NOT NULL DEFAULT 0,
    outfit_type SMALLINT NOT NULL
);
CREATE TABLE IF NOT EXISTS outfit_clothes (
    outfit_id SMALLINT NOT NULL REFERENCES outfits(outfit_id) ON DELETE CASCADE,
    clothe_id SMALLINT NOT NULL REFERENCES clothes(clothe_id) ON DELETE CASCADE,
    PRIMARY KEY (outfit_id, clothe_id)
);