
# Marco

A simple, single-server Discord bot made for friends.

Marco has several personalities. He'll switch between them when
certain trigger words are said. The exact list of trigger words is
available in the code, but my intent is that you don't look at that :)

Marco only responds to messages that directly mention him (either with
a Discord ping, a Discord reply, or with the text "Marco"). But he
listens (for trigger words) on all messages, even if he doesn't reply
to them.

This bot expects a few environment variables to exist:
* `DISCORD_TOKEN` shall be the bot's Discord token.
* `OPENAI_API_KEY` shall be the OpenAI API key.
* `DISCORD_DEBUG_GUILD_ID` shall be the guild ID of the server being
  used for debugging. This is **ONLY** relevant when running in debug
  mode (hence, using a single usually-private server for debugging).
  In release mode, this variable is ignored.

This bot is available under the [MIT License](LICENSE.txt).

## Marco's Friends

* [RolyBot](https://github.com/jbax1899/RolyBot/)
* [flukebot](https://github.com/EvanSkiStudios/flukebot)
* [Shawbot](https://github.com/Circaurus/Shawbot)
* [Beelzebot](https://github.com/Lukanibal/Beelzebot/)
