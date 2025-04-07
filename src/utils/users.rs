use teloxide::types::User;

pub fn get_user_str(user: Option<User>) -> String {
    user.as_ref()
        .map(|user| {
            user.mention()
                .map(|mention| mention.to_string())
                .unwrap_or_else(|| format!("{}", user.first_name))
        })
        .unwrap_or_else(|| "未知用户".to_string())
}
