#!/bin/sh
### BEGIN INIT INFO
# Provides:          testone
# Required-Start:    $local_fs
# Required-Stop:     $local_fs
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# X-Interactive:     false
# Short-Description: Example init script
# Description:       Start/stop an example script
### END INIT INFO

DESC="notification script"
NAME=testone
#DAEMON=

do_start()
{
    if [ $( [ ! -e log.txt ]; echo $? ) = 1 ] 
    then
        echo "The service is already running"
    else
        #Run the jar file MATH service
        # $JAVA -jar /opt/MATH/MATH.jar > /dev/null 2>&1 &
        #sleep time before the service verification
        sleep 5
        $TELL > log.txt
        #Verify if the service is running
        # $PGREP -f MATH  > /dev/null
        # VERIFIER=$?
        if [ $ZERO = $VERIFIER ]
        then
            echo "Service was successfully started"
        else
            echo "Failed to start service"
        fi
    fi
    echo
}

do_stop()
{
   echo "stopping!"
   rm -rf log.txt
}


case "$1" in
   start)
     do_start
     ;;
   stop)
     do_stop
     ;;
esac

exit 0