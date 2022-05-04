# lite-xl-uwuifier

[uwu'd readme](readme_uwu.md)

## this plugin has only been tested on linux. 

If you want to make it work on another OS, it probably won't be that hard. Just look in the makefile

### Installing the plugin
> Make sure you have lite-xl >= 2.1
> rust, gcc, cargo

run `make install`

it will automatically copy the stuff into your plugins folder

### usage
Right-click and choose uwuify on any doc. You can also run doc:uwuify

It will uwuify your selection, or the whole doc if nothing is selected.


### how?
this is just a lua binding made from a C binding I made of [this rust library](https://github.com/Daniel-Liu-c0deb0t/uwu)
