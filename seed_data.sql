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
DEFINE FIELD duration ON reservation TYPE number DEFAULT 2;
DEFINE FIELD day ON reservation TYPE datetime;
DEFINE FIELD location ON reservation TYPE record<location>;
DEFINE FIELD reserved_by ON reservation TYPE option<record<user>>;
DEFINE INDEX slot ON reservation FIELDS start, location, day UNIQUE;

CREATE reservation CONTENT {
    day: <datetime>"2024-12-01T13:00:00-06:00",
    location: location:chuys,
};
CREATE reservation CONTENT {
    day: <datetime>"2024-12-02T13:00:00-06:00",
    location: location:chuys,
};
CREATE reservation CONTENT {
    day: <datetime>"2024-12-01T15:00:00-06:00",
    location: location:walgreens,
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

DEFINE FUNCTION fn::day_of_week($date: datetime) {
LET $day = time::wday($date);
LET $name=
     IF $day = 1 { 'Monday'; }
ELSE IF $day = 2 { 'Tuesday'; }
ELSE IF $day = 3 { 'Wednesday'; }
ELSE IF $day = 4 { 'Thursday'; }
ELSE IF $day = 5 { 'Friday'; }
ELSE IF $day = 6 { 'Saturday'; }
ELSE IF $day = 7 { 'Sunday'; }
ELSE { THROW 'Invalid Day'; };
RETURN {
	day: $day,
	name: $name
};
};
