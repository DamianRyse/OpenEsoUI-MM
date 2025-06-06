# OpenESOUI-MM
[![Support Server](https://img.shields.io/discord/409050120894545920?color=%23ef5600&label=DISCORD&style=for-the-badge)](https://discord.gg/YP4eNUF)

The OpenESOUI-MM is an open-source and lightweight mod manager/addon manager alternative (in fact it's a web scraper) to the official client.
It's completely terminal based, written in the Rust programming language and works on Windows and Linux.

## How does it work?
When running the executable for the first time, it'll generate a configuration file in JSON format. The location of the file depends on whether you're on Windows or Linux:
- Windows: `\My Documents\openesoui-mm\config.json`
- Linux: `~/.config/openesoui-mm/config.json`

The structure of the configuration file is super simple:
```json
{
  "target_directory": "/path/to/ESO/live/AddOns",
  "addon_ids": [
    4063,
    3501
  ]
}
```
*target_directory* describes the path to your desired directory where the downloaded addons should go into. Ideally, this is the ESO addon directory.

*addon_ids* is an array of numeric values which represent the AddOns on [ESOUI.com](https://www.esoui.com). The AddOn ID is part of the URL. For example:
- https://www.esoui.com/downloads/info3501-SimpleSkyshards.html - The ID is 3501

Edit the configuration file with your desired AddOn path and the AddOn IDs you want to keep up to date and that's it. Next time you're executing **OpenESOUI-MM**, it'll download and extract your favorite AddOns for you.

Additionally, if you want to install a number of Addons without editing your configuration file, you can use the following argument:
```
./openesoui-mm --download 150,237,3175
```
You can also specify another target directory:
```
./openesoui-mm --target ~/your/desired/AddOn-directory
```

## What's the benefit over using the official client?
First of all, it's completely open-source. Second, it's written in Rust, therefore a standalone binary, instead of a Java application, requiring the Java framework. Third: You can *cronjob* or somewhat automate the process of updating your ESO addons. And probably some other things, too.

But of course, if you're happy with the official client, that's totally fine!

## Screenshot
![](https://i.imgur.com/MKfSSPD.png)
