DEFINE TABLE location SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON location TYPE string;
DEFINE INDEX name ON location FIELDS name UNIQUE;

CREATE location:chuys SET name = "Chuy's";
CREATE location:walgreens SET name = "Walgreens";
CREATE location:walmart SET name = "Walmart";

DEFINE TABLE day_of_week SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON day_of_week TYPE string;
DEFINE INDEX name ON day_of_week FIELDS name UNIQUE;

DEFINE TABLE reservation SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD start ON reservation TYPE number;
DEFINE FIELD duration ON reservation TYPE number DEFAULT 2;
DEFINE FIELD day_of_week ON reservation TYPE record<day_of_week>;
DEFINE FIELD location ON reservation TYPE record<location>;
DEFINE FIELD reserved_by ON reservation TYPE option<record<user>>;
DEFINE INDEX slot ON reservation FIELDS start, location, day_of_week UNIQUE;

CREATE reservation CONTENT {
    day_of_week: day_of_week:1,
    location: location:chuys,
    start: 13,
};
CREATE reservation CONTENT {
    day_of_week: day_of_week:2,
    location: location:chuys,
    start: 13,
};
CREATE reservation CONTENT {
    day_of_week: day_of_week:1,
    location: location:walgreens,
    start: 15,
};

DEFINE TABLE user SCHEMAFULL
PERMISSIONS
    FOR select, update, delete WHERE id = $auth.id;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD password ON user TYPE string;
DEFINE INDEX username ON user FIELDS username UNIQUE;

DEFINE ACCESS user ON DATABASE TYPE RECORD
    SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password) )
    SIGNIN ( SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password) );

CREATE day_of_week:1 CONTENT {name: "Monday"};
CREATE day_of_week:2 CONTENT {name: "Tuesday"};
CREATE day_of_week:3 CONTENT {name: "Wednesday"};
CREATE day_of_week:4 CONTENT {name: "Thursday"};
CREATE day_of_week:5 CONTENT {name: "Friday"};
CREATE day_of_week:6 CONTENT {name: "Saturday"};
CREATE day_of_week:7 CONTENT {name: "Sunday"};

LET $username="Brian";
LET $password="abc123";

CREATE user SET username=$username, password=crypto::argon2::generate($password);

