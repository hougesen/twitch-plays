# twitch-plays

A for fun Twitch Plays implementation in Rust using Twitch's websockets.

## Commands

The following commands are implemented:

### Game commands

-   !up
-   !down
-   !left
-   !right
-   !a
-   !b
-   !select
-   !start

### Channel commands

-   !github / !code
-   !help / !commands

## Disclaimer

Please note that, while this is a working implementation of Twitch Plays, you will have to build your own Twitch authentication flow. The flow will have to request at minimum the `chat:read` scope. If you wish to send chat messages from the bot you will need the `chat:edit` scope too.
