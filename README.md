# volume-notify-rs
This is a simple program for indicating current volume by sending a notification

### **Only works with wpctl !!!**


### Usage:
on niri, i execute it like this:
```
binds {
    XF86AudioRaiseVolume allow-when-locked=true { spawn-sh "wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.05+ -l 1.2 && ~/.config/niri/volume-notify-rs"; }
    XF86AudioLowerVolume allow-when-locked=true { spawn-sh "wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.05- && ~/.config/niri/volume-notify-rs"; }
    XF86AudioMute        allow-when-locked=true { spawn-sh "wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle && ~/.config/niri/volume-notify-rs"; }
}
```
also, i use mako and this is what i put in its config:
```
[app-name=volume-notify-rs]
layer=overlay
history=0
anchor=top-center
text-alignment=center
group-by=app-name
format=<b>%s</b>\n%b
```
