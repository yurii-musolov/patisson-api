SELECT
  id,
  username,
  password_hash
FROM
  users
WHERE
  username = $1
LIMIT
  1;