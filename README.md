# <a name="intro"></a>**Echo - a simple discord music bot**

This is an entirely self contained discord bot written in Rust, using [Serenity](https://serenity-rs.github.io/serenity/current/serenity/index.html) and [Songbird](https://serenity-rs.github.io/songbird/current/songbird/index.html).

## <a name="links"></a>Links
- [Installation](#install)

# <a name="install"></a>How to install

Once you have cloned the repository, you will need to get your bot a [token](https://discord.com/developers/applications). You can then copy this token and place it in [.envexample](/.envexample).

Then, rename [.envexample](/.envexample) to `.env`.

Now your bot (should be) working! You can invite it to your server using [this tool](https://discordapi.com/permissions.html).

In order to play music through this bot, you will also need to install [yt-dlp](https://github.com/yt-dlp/yt-dlp).

This is also packaged in some repos:

```bash
pacman -S yt-dlp
```

```bash
choco install yt-dlp
```