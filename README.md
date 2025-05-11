# GTokenChecker-rs

Re-implementation of the original GTokenChecker in Rust

## Features

- Fast checking
- Checks:
    - basic user information (username, id, avatar, banner, banner color, e-mail, phone, mfa enabled, bio)
    - user connections
    - relationships (all friends/blocked users + basic info about them)
    - user guilds (id, name, is a user is owner, user permissions, icon, banner, member_count)
    - current promotions
    - nitro information
        - basic nitro information (start time, end time, cancellation time, nitro type)
        - available boosts (is currently used, subscription id, guild id, is cancelled, cooldown expiration date)
        - available nitro credits

## How to use:

    1. Download the executable from [releases](https://github.com/nixxoq/gtokenchecker-rs/releases/latest), depending on
       your operating system and architecture

    2. Run terminal (windows terminal/powershell/cmd.exe on Windows, *sh based on linux)

    3. Type `gtokenchecker-rs-* --help` to get help and fill required values

    4. And that's all

## Future updates / TODO:

### 1.3

- [ ] implement the `check_payments` function (get available credit cards on the account)
- [ ] implement the `get_dms` function (get all direct messages from account)

### 1.4

- [ ] implement friendly TOML-based configuration (should be better than in older versions)