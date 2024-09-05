# Weather Stats Scripts

Welcome to the Weather Stats Service Scripts directory. This directory contains multiple useful scripts. You will see references to these scripts in the main [README](../README.md).


### build.sh
The build script is an important tool used to spin up a local version of the service. A succesful run of this script results in 2 running docker containers connected by a network. One container hosts a DB and the other hosts the application code. To avoid failed runs, read all instructions and adjust ENV variables accordingly.


### image_build.sh
The image build script is a tool used to create docker images ready to be pushed to Docker Hub. This script is esssential for adjusting application code. Cloud based versions of the sevice can only be updated using this method. Once again, be mindful of ENV variables to avoid failed runs.


### init.sql
The SQL init script contains starter SQL code for creating the service back end. Cloud based implementations will require these statements to be manually run on the cloud DB. For local implementations, the build script uses this script to set up the Docker based back end.