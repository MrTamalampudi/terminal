pub mod defines {
    pub const AUTH_ERROR_CODE: i32 = 11001;
    pub const SUBS_ERROR_CODE: i32 = 11011;
    pub const UNSUBS_ERROR_CODE: i32 = 11012;
    pub const RESUME_ERROR_CODE: i32 = 11032; // Note: 11031 was duplicated in Python, using the latter
    pub const MODE_ERROR_CODE: i32 = 12001;
    pub const SUCCESS_CODE: i32 = 200;
    pub const INVALID_CODE: i32 = -300;
    pub const TOKEN_EXPIRED: i32 = -99;
    pub const LIMIT_EXCEED_CODE: i32 = -99;

    pub const LIMIT_EXCEED_MSG_5000: &str = "Please provide less than 5000 symbols";
    pub const TOKEN_EXPIRED_MSG: &str = "Token is expired";
    pub const INVALID_TOKEN: &str = "Please provide valid token";

    pub const SUCCESS: &str = "ok";
    pub const ERROR: &str = "error";
    pub const AUTH_SUCCESS: &str = "Authentication done";
    pub const AUTH_FAIL: &str = "Authentication failed";

    pub const SUBSCRIBE_SUCCESS: &str = "Subscribed";
    pub const SUBSCRIBE_FAIL: &str = "subscription failed";
    pub const UNSUBSCRIBE_SUCCESS: &str = "Unsubscribed";
    pub const UNSUBSCRIBE_FAIL: &str = "unsubscription failed";

    pub const LITE_MODE: &str = "Lite Mode On";
    pub const FULL_MODE: &str = "Full Mode On";
    pub const MODE_CHANGE_ERROR: &str = "Mode change failed";

    pub const CHANNEL_PAUSED: &str = "Channel Paused";
    pub const CHANNEL_RESUMED: &str = "Channel Resumed";
    pub const CHANNEL_CHANGE_FAIL: &str = "Mode change failed";

    pub const INVALID_SYMBOLS: &str = "Please provide a valid symbol";
    pub const CONNECTION_CLOSED: &str = "Connection Closed";
    pub const INDEX_DEPTH_ERROR_MESSAGE: &str = "Index does not have market depth";

    pub const AUTH_TYPE: &str = "cn";
    pub const SUBS_TYPE: &str = "sub";
    pub const UNSUBS_TYPE: &str = "unsub";
    pub const LITE_MODE_TYPE: &str = "lit";
    pub const FULL_MODE_TYPE: &str = "ful";
    pub const CH_PAUSE_TYPE: &str = "cp";
    pub const CH_RESUME_TYPE: &str = "cr";
}
