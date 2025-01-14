CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password BYTEA NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS colors (
    color_id SERIAL PRIMARY KEY,
    color_name VARCHAR(50) UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS categories (
    category_id SERIAL PRIMARY KEY,
    category_name VARCHAR(50) UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS clothing_items (
    clothing_item_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    color_id INT NOT NULL REFERENCES colors(color_id),
    category_id INT NOT NULL REFERENCES categories(category_id),
    user_id INT NOT NULL REFERENCES users(user_id),
    is_hot_weather BOOLEAN DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS outfits (
    outfit_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id INT NOT NULL REFERENCES users(user_id)
);
CREATE TABLE IF NOT EXISTS outfit_clothing_items (
    outfit_id INT NOT NULL REFERENCES outfits(outfit_id) ON DELETE CASCADE,
    clothing_item_id INT NOT NULL REFERENCES clothing_items(clothing_item_id) ON DELETE CASCADE,
    PRIMARY KEY (outfit_id, clothing_item_id)
);
INSERT INTO colors (color_name)
VALUES ('Red'),
    ('Orange'),
    ('Yellow'),
    ('Green'),
    ('Blue'),
    ('Purple'),
    ('Pink'),
    ('Brown'),
    ('Black'),
    ('White'),
    ('Gold'),
    ('Gray') ON CONFLICT DO NOTHING;
INSERT INTO categories (category_name)
VALUES ('Shirt'),
    ('Pants'),
    ('Shorts'),
    ('Dress'),
    ('Skirt'),
    ('Jacket'),
    ('Sweater'),
    ('Shoes'),
    ('Hat'),
    ('Gloves'),
    ('Scarf') ON CONFLICT DO NOTHING;
INSERT INTO users (username, email, password, is_admin)
VALUES ('admin', 'admin@gmail.com', '1234', TRUE) ON CONFLICT DO NOTHING;