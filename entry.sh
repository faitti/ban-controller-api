#!/bin/sh
source .env
while [ 1 ];
do
    ./diesel database setup && break;
done
./diesel migration run
./ban-controller-api