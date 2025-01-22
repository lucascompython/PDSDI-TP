pub struct User {
    pub user_id: i16,
    pub username: String,
    pub email: String,
    pub password: [u8; 48], // 16 bytes for the salt and 32 bytes for the hash
    pub is_admin: bool,
}
