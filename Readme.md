# Hello
# Welcome to gin a package maeneger

It is work in progress.

For now, it only supports resolving dependencies froma project. 

To play around with it, you can start with this:

## Create a  file called "Ginfile"
## Then, put this in the gin file:

```
installationCommands:
- echo MUHAHAHAHAHAAHa
- touch I
testCommands:
- test -f I
dependencyFiles:
- https://github.com/jakobneufeld/Swift-Adons.git
source: https://github.com/jakobneufeld/Calc.git
name: <project name when download from the a index is working>
# Put your conflicting deps. There is no guarantee thta this will work
conflicts:
- h
```
[#] Build system deps
[] conflicts
[] more... 