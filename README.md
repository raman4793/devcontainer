## devcontainer

This is a personal project, the aim is two-fold. The first one is to help me improve my
workflow. I really like the idea of devcontainers. I couldn't find any cli that would
help me work with devcontainers. The only option was to use vscode or one of the jetbrains
ide to build and manage devcontainers. What I wanted was a cli tool that works with the 
devcontainer spec. The second aim to learn rust. This project didn't have any
requirements that warrented a specific language(other than the fact that it should have
an implementation of the docker API). I wasnt trying to write my own as it could be a 
project by itself.

These are the most important features that I needed from my devcontainer cli.

* Build devcontainer from the devcontainer.json file
* Run devcontainers with support for picking up the runArgs
* Easy mechanism to attach a tty to the devcontainer
* Support for the postCreateCommands
