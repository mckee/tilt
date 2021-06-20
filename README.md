# tilt
parses and displays iBeacon advertisements from [Tilt hydrometers](https://tilthydrometer.com/)

currently only prints results to stdout

## example setup on raspberry pi

### prerequisites
 - [Docker](https://docs.docker.com/engine/install/ubuntu/)
 - bluez
    - `sudo apt install bluez pi-bluetooth`

### example usage
```
$ sudo docker run --network=host --privileged -ti mckee/tilt
2021-06-20 21:12:59.764414765 UTC - PINK: 77°F, SG1.000
2021-06-20 21:13:03.791431565 UTC - PINK: 77°F, SG1.000
2021-06-20 21:13:05.795688853 UTC - PINK: 77°F, SG1.000
2021-06-20 21:13:06.798120817 UTC - PINK: 77°F, SG1.000
2021-06-20 21:13:08.801508763 UTC - PINK: 77°F, SG1.000
2021-06-20 21:13:17.833349296 UTC - PINK: 77°F, SG1.000
^C
```


