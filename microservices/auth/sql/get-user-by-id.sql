SELECT
  id,
  username,
  password_hash
FROM
  users
WHERE
  id = $1
LIMIT
  1;