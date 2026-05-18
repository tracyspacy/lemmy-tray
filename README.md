# lemmy-tray - read lemmy posts in system tray

A lightweight menu bar (system tray) app for Lemmy.

<img width="633" height="243" alt="Screenshot 2026-05-14 at 17 36 43" src="https://github.com/user-attachments/assets/97970c96-ee0d-47f4-9a6e-0a73ed2ea1f1" />

### Features
- Fetches one post with specified interval from a Lemmy instance and displays in menu bar. 
- Shows basic information about post: upvotes, downvotes, amount of comments etc in tray menu.
- Opens link to post if clicked.
- Sort type and listing (local/all) can be selected in app settings
- **no account is required**


### Build
```cargo build --release``` 
```./target/release/lemmy-tray```

### Configuration
Edit `config.toml` file in the project root.
```toml
[api_config]
instance = "lemmy.ml"       # instance address here
listing_type = "Local"      # listing types: Local or All
sort_type = "Active"        # sorting types: New, Active, Hot, TopDay, TopHour, NewComments, Scaled, Controversial

[app_config]
title_len_chars = 27        # length of title you will see in menu bar/ tray icon, default value is 27 chars
refresh_tick_sec = 60       # refresh interval, default value is 60 seconds
```
If configs are invalid, default configs will be used.

### Platforms
Tested on MacOs only, but probably may work on Linux/Win as well

