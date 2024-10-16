DEFINE TABLE location SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON location TYPE string;
DEFINE INDEX name ON location FIELDS name UNIQUE;

CREATE location:chuys SET name = "Chuy's";
CREATE location:walgreens SET name = "Walgreens";
CREATE location:walmart SET name = "Walmart";

DEFINE TABLE dayofweek SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON dayofweek TYPE string;
DEFINE INDEX name ON dayofweek FIELDS name UNIQUE;

CREATE dayofweek:monday SET name = "Monday";
CREATE dayofweek:tuesday SET name = "Tuesday";
CREATE dayofweek:wednesday SET name = "Wednesday";
CREATE dayofweek:thursday SET name = "Thursday";
CREATE dayofweek:friday SET name = "Friday";
CREATE dayofweek:saturday SET name = "Saturday";
CREATE dayofweek:sunday SET name = "Sunday";

DEFINE TABLE timeslot SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD start ON timeslot TYPE number;
DEFINE FIELD end ON timeslot TYPE number;
DEFINE INDEX slot ON timeslot FIELDS start, end UNIQUE;
CREATE timeslot SET start=1, end=3;
CREATE timeslot SET start=3, end=5;

DEFINE TABLE user SCHEMAFULL
PERMISSIONS
    FOR select, update, delete WHERE id = $auth.id;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD password ON user TYPE string;
DEFINE INDEX username ON user FIELDS username UNIQUE;

DEFINE ACCESS user ON DATABASE TYPE RECORD
    SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password) )
    SIGNIN ( SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password) );
