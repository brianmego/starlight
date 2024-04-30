DEFINE TABLE location SCHEMAFULL
PERMISSIONS
    FOR select WHERE $scope="user";
DEFINE FIELD name ON location TYPE string;
DEFINE INDEX name ON location FIELDS name UNIQUE;

CREATE location:chuys SET name = 'Chuys';
CREATE location:walgreens SET name = 'Walgreens';


DEFINE TABLE user SCHEMAFULL
PERMISSIONS
    FOR select, update, delete WHERE id = $auth.id;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD password ON user TYPE string;
DEFINE INDEX username ON user FIELDS username UNIQUE;

DEFINE SCOPE user SESSION 24h
    SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password) )
    SIGNIN ( SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password) );

DEFINE SCOPE loser SESSION 24h
    SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password) )
    SIGNIN ( SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password) );
