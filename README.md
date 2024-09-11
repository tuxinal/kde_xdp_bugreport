# xdg-desktop-portal-kde bug report

requires rust to be installed and requires kde's xdg-portal-backend to be running

run with `kioclient exec kde-xdp-bugreport.desktop` to get the correct app name while in KCM (otherwise will use the calling applications app_id)

creates two shorcuts with the [GlobalShortcuts](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html) api. quits when `ALT+n` is pressed (by default).

expected behaviour: after creating the shorcuts in the first run, subsequent runs will use the shorcuts from the first run.

observed behaviour: runs cannot reuse the shorcuts from previous runs. thus making a new set of shorcuts each time. this does not remove the old shorcuts and the new shorcuts will be disabled by default due to conflicts.
