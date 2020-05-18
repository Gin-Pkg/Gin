# Hello
# Welcome to gin a package maeneger

It is work in progress.

For now, it only supports resolving dependencies froma project. 

To play around with it, you can start with this:

## Create a  file called "Ginfile"
## Then, put this in the gin file:

```yaml
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

# Todo
- [x]  Build system deps
- [ ] conflicts
- [ ] more...  


I am currently working on the indexing so you can install packages without downloading the entire Ginfile. Please contact me at jakob.n.neufeld@gmail.com if there are any problems.


## Look at my git orginization which currently have two repos
 - One called "Gin" are the sources for the package manager. It also has an example
 - One called "Gin-Index" which is work ij  progress.

 ### Put any issues on the source repo. Not on the index repo.


 # Here is the link to my git org: [Git Repo Org](https://github.com/Gin-Pkg/)
