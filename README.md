# nag-rs

A weekend rust project to learn the language, replaces some bash scripts I have
been using to create ephemeral alerts (using `i3-nagbar` and `paplay`).  Probably
not very useful to anyone else.

 - Start `nagd` or run it as a daemon.
 - use `nag` to communicate with it:
    - `add` a nag
       - duration in the form of '-d-h-m-s' such as '1d' or '5h3m' or '4m5s'
       - a name to display in the nag bar
       - an optional path to a local sound file to pass to paplay.
       - ex: `nag add 1h "YOUR TOTINOS™ PIZZA ROLLS ARE BURNING!" "~/sounds/campfire.wav"`
   - `list` will print out the list of nags currently active
       - ex: `nag list`
   - `edit` nags will open up nvim and allow you to add, remove, or edit nags.
       - ex: `nag edit`

When the timer is up, the `i3-nagbar` is triggered and the sound plays.  if you
dismiss the bar, the sound will stop.
