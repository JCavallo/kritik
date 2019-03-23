kritik is a program designed to "nicely" run programs in the command line. It
was inspired by `chronic`, which is included in the
[moreutils](https://joeyh.name/code/moreutils/) tools, and "runs a command
quietly unless it fails".

This is also my very first project in Rust, so it is probably far from
perfect :).

Basic usage would be for instance:

`` kritik git pull ``

It is possible to set a message that will be displayed while the command runs:

`` kritik --message "Updating" git pull ``

Chained commands can be called with quotes:

`` kritik "git fetch -p origin && git merge origin/master" ``

The output will be the message and a spinner while the command runs. The
standard output / errors will only be shown if the commands exits with a
non-zero error code.

Available options can be listed with:

`` kritik --help ``
