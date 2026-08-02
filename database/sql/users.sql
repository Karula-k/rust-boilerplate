--! get_users
SELECT * FROM users
WHERE users.id = :id LIMIT 1;

--! list_users
SELECT id,username,created_at,updated_at FROM users
ORDER BY id
LIMIT :limit
OFFSET :offset;

--! create_users
INSERT INTO users(
  username,
  password
) VALUES (
  :username,:password
)RETURNING *;

--! delete_user
DELETE FROM users
WHERE users.id = :id;

--! update_user
UPDATE users
SET username = :username,
    password = :password,
    updated_at = NOW()
WHERE users.id = :id;

--! get_user_by_username
SELECT * FROM users
WHERE users.username like CONCAT(:star_str::text,'%');