# Ampersand Roadmap
## 0.2.0 to 1.0.0
In Ampersand 0.2.0, the API simply wraps Rusts GTK3 crate. I can see this will possibly have issues, however the reason I wanted it to be like this was simply so if the graphics, or windows for any app (such as Underscore, Emdash etc) broke, instead of having to push a patch to each app, I just updated the wrapper.

I'm personally not very happy with this solution, which is why I plan to turn Ampersand into its own window drawing library, similar but seperate from GTK.

In future, Ampersand will be its own API built using Cairo for drawing, and the X11 library for input. These changes will be in place <i>before</i> MAJOR release 1.
I do not intend for there to be Wayland support for Ampersand, however if by some miracle this library does get some attention and there is demand for Wayland support, I will see what I can do.

As for what features I intend Ampersand to have, I want it to have everything a developer might need. Drawing windows, handling input, maybe even audio at some point (though that might live in a different crate).
This is a big project, and is pretty ambitious. I doubt I would be able to finish this project on my own, which is why I do request assistance from those in the however small or large community. 