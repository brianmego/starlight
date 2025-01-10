--Location
DEFINE TABLE location SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON location TYPE string;
DEFINE FIELD address ON location TYPE string;
DEFINE FIELD notes ON location TYPE string;
DEFINE INDEX name ON location FIELDS name UNIQUE;

--DayOfWeek
DEFINE TABLE day_of_week SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD name ON day_of_week TYPE string;
DEFINE INDEX name ON day_of_week FIELDS name UNIQUE;

--Reservation
DEFINE TABLE reservation SCHEMAFULL
    PERMISSIONS FOR select WHERE $access="user";
DEFINE FIELD duration ON reservation TYPE number DEFAULT 2;
DEFINE FIELD day ON reservation TYPE datetime;
DEFINE FIELD location ON reservation TYPE record<location>;
DEFINE FIELD reserved_by ON reservation TYPE option<record<user>>;
DEFINE INDEX slot ON reservation FIELDS start, location, day UNIQUE;

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
DEFINE INDEX name ON trooptype FIELDS name UNIQUE:

-------Data-------
CREATE trooptype:level1 SET name="Level1";
CREATE trooptype:level2 SET name="Level2";
CREATE trooptype:level3 SET name="Level3";


CREATE location:chuys SET name = "Chuy's";
CREATE location:fine_eyewear SET name = "Fine Eyewear";
CREATE location:h_mart SET name = "H-Mart";
CREATE location:kirklands SET name = "Kirkland's";
CREATE location:lowes_620 SET name = "Lowes 620";
CREATE location:mega_furniture SET name = "Mega Furniture";
CREATE location:mighty_fine SET name = "Mighty Fine";
CREATE location:old_navy SET name = "Old Navy";
CREATE location:papa_murphys SET name = "Papa Murphy's";
CREATE location:phonatic SET name = "PhoNatic";
CREATE location:randalls_cafe SET name = "Randall's Cafe";
CREATE location:randalls_floral SET name = "Randall's Floral";
CREATE location:smokey_mos SET name = "Smokey Mo's";
CREATE location:tony_cs SET name = "Tony C's";
CREATE location:walgreens_avery_ranch SET name = "Walgreens Avery Ranch";
CREATE location:walgreens_cypress_creek SET name = "Walgreens Cypress Creek";
CREATE location:walgreens_discovery SET name = "Walgreens Discovery";
CREATE location:walmart_1431_market SET name = "Walmart 1431 Market";
CREATE location:walmart_1431_pharmacy SET name = "Walmart 1431 Pharmacy";
CREATE location:walmart_620 SET name = "Walmart 620";
CREATE location:walmart_grocery SET name = "Walmart Grocery";
CREATE location:walmart_walton_way SET name = "Walmart Walton Way";
CREATE location:walmart_walton_way_grocery SET name = "Walmart Walton Way Grocery";

let $jan_weekends = ["18", "19", "25", "26"];
let $jan_weekdays = ["20", "21", "22", "23", "24", "27", "28", "29", "30", "31"];
let $feb_weekends =  ["01", "02", "08", "09", "15", "16", "22", "23"];
let $feb_weekdays = ["03", "04", "05", "06", "07", "10", "11", "12", "13", "14", "17", "18", "19", "20", "21"];

FOR $location in (select * from location where not(name.starts_with("Randall's") or name.starts_with("Walmart"))) {
    #January
    FOR $day in $jan_weekends {
        FOR $hour in ["09", "11", "13", "15", "17", "19"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-01-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    };
    FOR $day in $jan_weekdays {
        FOR $hour in ["15", "17", "19"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-01-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    };

    #February
    FOR $day in $feb_weekends {
        FOR $hour in ["09", "11", "13", "15", "17", "19"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-02-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    };
    FOR $day in $feb_weekdays {
        FOR $hour in ["15", "17", "19"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-02-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    }
};

FOR $location in (select * from location where name.starts_with("Randall's") or name.starts_with("Walmart")) {
    #January
    FOR $day in $jan_weekends {
        FOR $hour in ["10", "12", "14", "16", "18", "20"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-01-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    };
    FOR $day in $jan_weekdays {
        FOR $hour in ["16", "18", "20"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-01-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    };

    #February
    FOR $day in $feb_weekends {
        FOR $hour in ["10", "12", "14", "16", "18", "20"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-02-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    };
    FOR $day in $feb_weekdays {
        FOR $hour in ["16", "18", "20"] {
            CREATE reservation CONTENT {
                day: <datetime>string::concat("2025-02-", $day, "T", $hour, ":00:00-06:00"),
                location: $location.id,
            }
        };
    }
};


CREATE day_of_week:1 CONTENT {name: "Monday"};
CREATE day_of_week:2 CONTENT {name: "Tuesday"};
CREATE day_of_week:3 CONTENT {name: "Wednesday"};
CREATE day_of_week:4 CONTENT {name: "Thursday"};
CREATE day_of_week:5 CONTENT {name: "Friday"};
CREATE day_of_week:6 CONTENT {name: "Saturday"};
CREATE day_of_week:7 CONTENT {name: "Sunday"};

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
