<a href='https://ko-fi.com/A0A8Q3SVZ' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://storage.ko-fi.com/cdn/kofi4.png?v=3' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>
# Discord Rich Presence for Creative Apps
### Show your friends what you're working on, be it in Adobe Suite, Autodesk Suite, Cinema 4D or many more!
This app runs in the background and looks for processes, then parses the windows title and turns it into a project name to display with Discords Rich Presence.

Examples:
![example1](https://i.imgur.com/yFzQh6O.png)
![example2](https://i.imgur.com/fziotzt.png)
![example3](https://i.imgur.com/9SEXuQr.png)
![example4](https://i.imgur.com/ANAW6Ub.png)
## how to install:
1. Grab the latest version from the [Releases](https://github.com/djkato/DRP_Creative/releases) page, then make a folder in `C:/Program Files/` and call it whatever, for example `DRP Creative`.
3. Put the exe inside the folder you created and create a link to it.
4. Do `[WindowsButton] + R`, type `Shell:StartUp`, hit enter. A folder will open.
5. Take the link file and put it inside the startup folder (Now the app will start on pc start)
6. If you want to run it now, double click it
7. Enjoy!

## How to use:
-   App updates will be notified and suggested on startup via popup window.
-   When it's running, you'll notice a new icon appear on your Taskbar. If you want to include/exclude the **currently open project** from showing up, click on the icon and click on `Remove project from include/exclude list`.
- There are two modes for the keywords list:
  1.  Exclude(default): If the currently open project files name contains the keyword anywhere, it will show the default project name instead.
  2. Include: All projects will show default names, unless the the currently open project files name contains a keyword, then it will display the real name. This is a good alternative if you find yourself hiding more than showing what you're working on.
- If you don't want to see the default project names, and instead just not display anything, you can change that behavior by setting `show_default_when_excluded` to `false` in the config file.
- If you don't like the *"Portfolio: [your link]"* line on your status, you can disable it by setting `hide_portfolio_link` to `true`

Example `drp_config.toml`:
```toml
keywords_list = ["WORK", "Untitled  681*"]
show_default_when_excluded = true # or false
portfolio_link = "djkato.net"
hide_portfolio_row = false
should_list_include_or_exclude = "Include" # Or "Exclude", capital sensitive

```
### Currently supported programs:
**Full support:**
-   Cinema 4D
-   Adobe After Effects
-   Adobe Illustrator
-   Adobe Photoshop
-   Adobe Premiere Pro
-   Autodesk Maya
-   Autodesk 3Ds Max
-   Davinci Resolve
-   Isotropix Clarisse
-   Cavalry
-   Ableton
-   FL Studio
-   Blender

**Partial support:** *(meaning it shows up on Discord, but doesn't display the project name)*
-   Marvelous Designer ¹*
-   Substance Designer ¹*
-   Substance Painter  ¹*
-   Adobe Audition ¹*
-   ZBrush ¹*
-   Darktable ¹*

¹*sadly not doable, project name not in proces window title. They will show up though, just with default project name*
