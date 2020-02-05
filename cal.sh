#!/bin/bash
echo Date:
read date
echo Event:
read event 
echo Time:
read tim 

echo $event, at, $tim, $date

if [ -f "/home/$USER/code/rustful-notifications/events/$date.json" ]; then
    echo "$date exist"
    ADDEVENT="$(jq --arg m "$event" '.event[.event | length] |= . + $m' /home/$USER/code/rustful-notifications/events/$date.json > /home/$USER/code/rustful-notifications/events/$date.tmp && mv /home/$USER/code/rustful-notifications/events/$date.tmp /home/$USER/code/rustful-notifications/events/$date.json )"
    ADDTIME="$(jq '.time[.time | length] |= . + "'$tim'"' /home/$USER/code/rustful-notifications/events/$date.json > /home/$USER/code/rustful-notifications/events/$date.tmp && mv /home/$USER/code/rustful-notifications/events/$date.tmp /home/$USER/code/rustful-notifications/events/$date.json )"
else 
    echo "$date does not exist"
    echo '{ "event": [], "time": [] }' > "/home/$USER/code/rustful-notifications/events/$date.json"
    ADDEVENT="$(jq --arg m "$event" '.event[.event | length] |= . + $m' /home/$USER/code/rustful-notifications/events/$date.json > /home/$USER/code/rustful-notifications/events/$date.tmp && mv /home/$USER/code/rustful-notifications/events/$date.tmp /home/$USER/code/rustful-notifications/events/$date.json )"
    ADDTIME="$(jq '.time[.time | length] |= . + "'$tim'"' /home/$USER/code/rustful-notifications/events/$date.json > /home/$USER/code/rustful-notifications/events/$date.tmp && mv /home/$USER/code/rustful-notifications/events/$date.tmp /home/$USER/code/rustful-notifications/events/$date.json )"
fi