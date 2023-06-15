# Weather Dashboard

A weather dashboard originally created for the Kobo Mini (2012).

Tested on Kobo Mini firmware 3.19.5761-s, the latest available as described at 
https://pgaskin.net/KoboStuff/kobofirmware.html. Note that
this firmware runs glibc 2.15 so this project compiles against
musl to avoid any compatibility issues.

## rtcwake notes

A tweaked version of rtcwake is required to enable resuming from sleep on the Kobo Mini.

A version can be found here:
https://web.archive.org/web/20160401013708/http://www.scherello.de/rtcwake_kobo.zip

Sleep for an hour:
```sh
./busybox_kobo rtcwake -m mem -s 3600
```

## References

* [Kobo Touch Hacking](https://wiki.mobileread.com/wiki/Kobo_Touch_Hacking)
* [Hacking the Kobo Touch for Dummies](https://web.archive.org/web/20190206185343/http://www.chauveau-central.net/pub/KoboTouch/)
* [rtcwake Discussion](https://www.mobileread.com/forums/showthread.php?t=212145&highlight=rtcwake&page=5)
* [Getting suspend working](https://www.mobileread.com/forums/showpost.php?p=2828681&postcount=77)
