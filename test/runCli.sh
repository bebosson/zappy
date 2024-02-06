#!/bin/sh

if [ $# -ne 2 ]; then
    echo "Usage: ./run_client.sh <nb players by team> <team name>"
    exit 1
fi

# Vérifier si le deuxième argument est numérique
if ! [[ $1 =~ ^[0-9]+$ ]]; then
    echo "Usage: ./run_client.sh <nb players by team> <team name>"
    exit 1
fi

i=0
nb_teams=$(( $# - 2 ))
while [[ $i -lt $nb_teams ]]; do
    counter=0
    while [ $counter -lt $1 ]; do
        echo run player $counter for client ${@:2+$i}
        cargo run --bin client ${@:2+$i} test/simple.txt
        ((counter++))
    done
    ((i++))
done
