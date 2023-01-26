# Weather Dashboard

A weather dashboard original created for the Kobo Mini (2012).

Tested on Kobo Mini firmware 3.19.5761-s, the latest available as described at 
https://pgaskin.net/KoboStuff/kobofirmware.html. Note that
this firmware runs glibc 2.15 so this project compiles against
musl to avoid any compatibility issues.

## rtcwake

https://web.archive.org/web/20160401013708/http://www.scherello.de/rtcwake_kobo.zip
Sleep for an hour:
./busybox_kobo rtcwake -m mem -s 3600