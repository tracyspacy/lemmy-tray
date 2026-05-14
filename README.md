# lemmy-tray - read lemmy posts in system tray

A lightweight menu bar (system tray) app for Lemmy.

<img width="633" height="243" alt="Screenshot 2026-05-14 at 17 36 43" src="https://github.com/user-attachments/assets/97970c96-ee0d-47f4-9a6e-0a73ed2ea1f1" />

### Features
- Fetches one post with specified interval from a Lemmy instance and displays in menu bar. 
- Shows basic information about post: upvotes, downvotes, amount of comments etc in tray menu.
- Opens link to post if clicked.
- Sort type and listing (local/all) can be selected in app settings


### Build
```cargo build --release``` 
```./target/release/lemmy-tray```


### Platforms
Tested on MacOs only, but probably may work on Linux/Win as well

### Note
No config file yet, no easy way to change default settings.
