#!/bin/bash
echo Date:
read date
echo Event:
read event 
echo Time
read tim 

echo $event, at, $tim, $date

EVENTCM="$(jq '.event | length' /home/blujay/code/rustful-notifications/src/todo.json)" 
TIMECM="$(jq '.time | length' /home/blujay/code/rustful-notifications/src/todo.json)"

TIMECM=$((TIMECM+1))
EVENTCM=$((EVENTCM+1))

echo $EVENTCM $TIMECM