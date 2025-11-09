pub const AVAILABLE_RESERVATIONS_QUERY: &str = "
    SELECT
        time::format(day - 6h, '%Y-%m-%d') as date,
        id AS reservation_id,
        fn::day_of_week(day - 6h).day AS day_of_week_id,
        fn::day_of_week(day - 6h).name AS day_of_week_name,
        location AS location_id,
        location.name AS location_name,
        location.address AS location_address,
        location.notes AS location_notes,
        time::hour(day - 6h) AS start_time,
        day > $next_week_start as next_week
    FROM reservation
    WHERE reserved_by=None
      AND location.enabled=true
      AND day > $start_time
      AND day < $end_time
      AND day > $campaign_start
";

pub const USER_RESERVATION_QUERY: &str = "
    SELECT
        id AS reservation_id,
        time::format(day - 6h, '%Y-%m-%d') as date,
        fn::day_of_week(day - 6h).day AS day_of_week_id,
        fn::day_of_week(day - 6h).name AS day_of_week_name,
        location AS location_id,
        location.name AS location_name,
        location.address AS location_address,
        location.notes AS location_notes,
        time::hour(day - 6h) AS start_time,
        day > $next_week_start as next_week,
        day <= $current_time as passed
    FROM reservation
    WHERE reserved_by=$user
    ORDER BY date;
";

pub const SET_RESERVATION_QUERY: &str = "
    (
        UPDATE reservation
          SET reserved_by=$user
        WHERE id = $reservation_id
          AND reserved_by == None
    ).len()
";

pub const USER_TOKEN_USAGE_COUNT: &str = "
    (
        SELECT * from reservation
        WHERE reserved_by=$user
        AND day > $next_week_start
    ).len()
";

pub const USER_SWAP_RESERVATION: &str = "
    BEGIN TRANSACTION;

    UPDATE reservation
      SET reserved_by=$user
    WHERE id = $new_reservation_id
      AND reserved_by == None;

    UPDATE reservation
      SET reserved_by=None
    WHERE id = $old_reservation_id
      AND reserved_by == $user;

    COMMIT TRANSACTION;
";

pub const CLAIMED_RESERVATIONS: &str = "
    SELECT id,
        day as date,
        reserved_by.username as username,
        location.name as location
    FROM reservation
    WHERE reserved_by != None
    ORDER BY date DESC
";
