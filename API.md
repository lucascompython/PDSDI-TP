# API Documentation

## Endpoint: Check User

- **URL**: `/user/check`
- **Method**: `GET`
- **Description**: This endpoint verifies user authentication.

**Input (Request Body)**:

```json
{
  "name": "lucas",
  "age": 19
}
```

**Output (Response)**:

- **Success (200 OK)**

---

## Endpoint: Register User

- **URL**: `/user/register`
- **Method**: `POST`
- **Description**: This endpoint allows a new user to register in the system.

**Input (Request Body)**:

```json
{
  "username": "gui",
  "email": "gui@gmail.com",
  "password": "1234",
  "admin": true
}
```

**Output (Response)**:

- **Success (200 OK)**
- **Error (400 Bad Request)**
- **Error (500 Internal Server Error)**

---

## Endpoint: Login User

- **URL**: `/user/login`
- **Method**: `POST`
- **Description**: Allows a user to log into the system.

**Input (Request Body)**:

```json
{
  "email": "joao@gmail.com",
  "password": "1234"
}
```

**Output (Response)**:

- **Success (200 OK)**
- **Error (401 Unauthorized)**:

---

## Endpoint: Logout User

- **URL**: `/user/logout`
- **Method**: `POST`
- **Description**: Logs the user out and deletes their session cookie.

**Output (Response)**:

- **Success (200 OK)**

---

## Endpoint: Protected Route (Admin Only)

- **URL**: `/user/protected`
- **Method**: `GET`
- **Description**: A test route accessible only to admin users.

**Output (Response)**:

- **Success (200 OK)**
- **Error (403 Forbidden)**

---

## Endpoint: Get Clothes

- **URL**: `/clothes/get`
- **Method**: `GET`
- **Description**: Retrieves all clothing items associated with a given user.

**Output (Response)**:

- **Success (200 OK)**:
  List of clothing items in CBF format.
