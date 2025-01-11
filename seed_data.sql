--Location
DEFINE TABLE location SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON location TYPE string;
DEFINE FIELD address ON location TYPE string;
DEFINE FIELD notes ON location TYPE option<string>;
DEFINE INDEX name ON location FIELDS name UNIQUE;

--Reservation
DEFINE TABLE reservation SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD duration ON reservation TYPE number DEFAULT 2;
DEFINE FIELD day ON reservation TYPE datetime;
DEFINE FIELD location ON reservation TYPE record<location>;
DEFINE FIELD reserved_by ON reservation TYPE option<record<user>>;
DEFINE INDEX slot ON reservation FIELDS location, day UNIQUE;

-- Create a new event whenever a reservation reserved_by changes
DEFINE EVENT OVERWRITE reserved_by ON TABLE reservation WHEN $before.reserved_by != $after.reserved_by THEN (
    CREATE reservation_log SET
        reservation_id       = $value.id,
        // Turn events like "UPDATE" into string "reserved_by updated"
        action     = 'reserved_by' + ' ' + $event.lowercase() + 'd',
        // `reserved_by` field may be NONE, log as '' if so
        old_reserved_by  = $before.reserved_by ?? '',
        new_reserved_by  = $after.reserved_by  ?? '',
        at         = time::now()
);

--User
DEFINE TABLE user SCHEMAFULL
PERMISSIONS
    FOR select, update, delete WHERE id = $auth.id;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD password ON user TYPE string;
DEFINE FIELD trooptype ON user TYPE record<trooptype>;
DEFINE INDEX username ON user FIELDS username UNIQUE;
DEFINE ACCESS user ON DATABASE TYPE RECORD
    SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password) )
    SIGNIN ( SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password) );

-- TroopType
DEFINE TABLE trooptype SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON trooptype TYPE string;
DEFINE INDEX name ON trooptype FIELDS name UNIQUE;

-------Functions-------

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
	day: ($day + 1) % 7,
	name: $name
};
};
