#!/bin/bash
echo "Rustful Notifications"
echo '$0 = ' $0
echo '$1 = ' $1 #date
echo '$2 = ' $2 #event
echo '$3 = ' $3 #time

echo $2, at, $3, $1

if [ -f "/home/$USER/code/rustful-notifications/events/$1.json" ]; then
    echo "$1 exist"
    ADDEVENT="$(jq --arg m "$2" '.event[.event | length] |= . + $m' /home/$USER/code/rustful-notifications/events/$1.json > /home/$USER/code/rustful-notifications/events/$1.tmp && mv /home/$USER/code/rustful-notifications/events/$1.tmp /home/$USER/code/rustful-notifications/events/$1.json )"
    ADDTIME="$(jq '.time[.time | length] |= . + "'$3'"' /home/$USER/code/rustful-notifications/events/$1.json > /home/$USER/code/rustful-notifications/events/$1.tmp && mv /home/$USER/code/rustful-notifications/events/$1.tmp /home/$USER/code/rustful-notifications/events/$1.json )"
else 
    echo "$1 does not exist"
    echo '{ "event": [], "time": [] }' > "/home/$USER/code/rustful-notifications/events/$1.json"
    ADDEVENT="$(jq --arg m "$2" '.event[.event | length] |= . + $m' /home/$USER/code/rustful-notifications/events/$1.json > /home/$USER/code/rustful-notifications/events/$1.tmp && mv /home/$USER/code/rustful-notifications/events/$1.tmp /home/$USER/code/rustful-notifications/events/$1.json )"
    ADDTIME="$(jq '.time[.time | length] |= . + "'$3'"' /home/$USER/code/rustful-notifications/events/$1.json > /home/$USER/code/rustful-notifications/events/$1.tmp && mv /home/$USER/code/rustful-notifications/events/$1.tmp /home/$USER/code/rustful-notifications/events/$1.json )"
fi