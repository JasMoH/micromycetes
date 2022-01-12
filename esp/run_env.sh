#!/bin/bash

set -e

##################################################
# build parameter list
##################################################

# interactive
PARAMS+=" --interactive --tty"

#mount contents of project
PARAMS+=" -v $PWD/..:/usr/src/project"

#set the working directyr as the project directory
PARAMS+=" -w /usr/src/project/esp"

PARAMS+=" -v $HOME/.cargo/:$HOME/.cargo"
PARAMS+=" -e CARGO_HOME=$HOME/.cargo"

##################################################
# mount some user info
# normally docker provides user isolation, but it can be
# convinient to have your user outside the container inside the container
# so you have write permissions for files created in mounted volumes, sudo privildges, etc.
##################################################

#mount passwd files and run as current user
#this will permit use of sudo inside the container
PARAMS+=" -v /etc/passwd:/etc/passwd:ro"
PARAMS+=" -v /etc/group:/etc/group:ro"
PARAMS+=" -v  /etc/shadow:/etc/shadow:ro"
PARAMS+=" -u $(id -u):$(id -g)" #runs as current user

#mount x server for gui use
# PARAMS+=" -v /tmp/.X11-unix:/tmp/.X11-unix"
PARAMS+=" -e DISPLAY=${DISPLAY}"
# PARAMS+=" --device=/dev/video0:/dev/video0"
PARAMS+=" -v $HOME/.Xauthority:/$HOME/.Xauthority"

##################################################
# consider mounting other contents of your home directory, such as .bashrc, git preferences, etc.
# this can bring some of your working environment in to the container
##################################################

# there are a couple ways of mounting xauth permissions and displays, this is useful if the container is running
# on a remote server and will need to forward the xwindow over ssh

# PARAMS+=" -e HOST=cctv"
# DISPLAY_NUMBER=$(echo $DISPLAY | cut -d. -f1 | cut -d: -f2)
# AUTH_COOKIE=$(xauth list | grep "^$(hostname)/unix:${DISPLAY_NUMBER} " | awk '{print $3}')
# xauth add ${HOST}/unix:${DISPLAY_NUMBER} MIT-MAGIC-COOKIE-1 ${AUTH_COOKIE}
# python /path/to/program.py
# PARAMS+=" -e AUTH_COOKIE=${AUTH_COOKIE}"


# here are some common group and privilidge options that are neccesary for hardware access from time to time
# avoid using them if they aren't neccesary, and be vary careful about pushing them in to production
 PARAMS+=" --privileged"
# PARAMS+=" --group-add dailout"

 PARAMS+=" -v /dev:/dev"

# if don't want docker to provide network isolation
# PARAMS+=" --network=host"



##################################################
# run the container
##################################################

echo $PARAMS
# xhost +local:docker
docker run $PARAMS espressif/idf-rust
