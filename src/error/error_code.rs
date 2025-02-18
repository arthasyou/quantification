macro_rules! status_error_codes {
    // 匹配多个元组，生成多个常量
    (
        $(
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        $(
            // 为每个元组生成一个常量定义
            pub const $konst: (u16, &str) = ($num, $phrase);
        )+
    }
}

status_error_codes! {
    (1, SERVER_ERROR, "server error");
    (10, USER_EXIST, "user exist");
    (11, USER_NOT_EXIST, "user not exist");
    (12, PASSWORD_ERROR, "password error");
    (13, USER_NOT_LOGIN, "user not login");
    (14, USER_NOT_FOUND, "user not found");
    (15, USER_NOT_AUTHORIZED, "user not authorized");
    (16, USER_NOT_PERMISSION, "user not permission");
}
