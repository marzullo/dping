# dping (Discord Ping over HTTP)

This program provides a single HTTP endpoint (/notify/\<message>) to send a DM to a specific discord user.

The following variables must be defined in a .env file:
```
DISCORD_TOKEN (discord bot token)
WEB_PORT (port number)
USER_ID (the u64 discord user id)
```