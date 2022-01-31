# twitch-pal-rs


## Building / Running
Config (Requires Python3)
```
python scripts/config_ui.py
```

Avatars
```
cargo run --release
```


## Events
What triggers an event?
* Channel event eg. raid, donations etc
    * Polls eg. Dividing people into an area, based on poll answer
* User joining/leaving stream - entrance/exits
    * Note: This has to differ from a raid

How do we know about an event type?
* Use an enum to tell the UserHandler how to add, remove or set the behaviour of avatars

The event handler does not need to exist in the scene, it can live within the monitor since it tracks the data required. The monitor also contains the user_handler, so we do not (or at least should not) need to cache it within the event system. 


## Modifications / Fixes
* [ ] Convert some Strings to &str (where applicable)


## Potential Features / TODO
* [ ] Sprites
    * [ ] Cosmetics - eg. Random hats
    * [ ] Follower/Tier based cosmetics
    * [ ] Custom sprite sheets
* [x] Animation + Animation Controller | Depends on Resource manager
    * [x] Load all frames of animation and save to Animation + Resource Manager
* [x] Cache user's status (eg. Mod/VIP)
* [ ] Socket to get twitch messages, show them above their avatar (like Stream Avatars)
    - Requires OAUTH token
    * [ ] Restrict chat messages to a specific bounds, so it isn't just a long line.
        * Issue: Newlines are not considered, even when we add them in
        * Note: Maybe parse the message and insert a newline every Nth word
    * [ ] Chat to interact with the avatars
        * [ ] Commands
            * [ ] add/remove avatar
            * [ ] (Mod/Broadcaster) Reset avatars, remove all and respawn
        * [ ] Messages
            * [ ] Filter messages for bad words (customise these words in config)
                - Note: Not sure if automod will capture this, we might still receive it
            * [ ] Display chat message above their avatar
* [ ] Random events / occurances that changes state in some way
    * [ ] Polls interact with avatars, people who have voted stay in a section of the screen. (Maybe draw text in the screen sections to show the choice)
    * [ ] Channel events cause special FX (eg. fireworks when a raid occurs, maybe all the raiders run in from the side)
        * Note: If I'm confident enough, maybe airdrop or train them in
        * Airdrop: Plane flies by with Raider's name, dropping all their viewers into the field
    * [ ] Every Nth milestone, drop in a boss or something similar. Chatters have to type command !attack to battle the boss
        * [ ] Channel Points
            * [ ] Gain extra damage and/or defence
            * [ ] Temporary invincibility
    * [ ] End of stream events
        * [ ] If raid:
            * [ ] Plane lands, picks up everyone, flies away (kinda cool if other streamer has the program, makes it feel like a continuation)
        * [ ] Random disaster: Meteor shower, alien abduction
    * [ ] Random Entrance/Exit:
        * [ ] Dr who tardis appears, they walk int/out
        * [ ] Rick n Morty portal, they walk in/out
        * [ ] Fly in/out


## Blacklist.txt
credit: https://github.com/arrowgent/Twitchtv-Bots-List/blob/main/list.txt