# ppnd -- a prettier /proc/net/dev

Not much magic here. This application reads /proc/net/dev and returns its contents in a neat and concise format. It offers scaling and precision options for the
byte counts emitted by /proc/net/dev, and has the ability to repeatedly read /proc/net/dev (with a user-configured delay between reads). Here is the output from `ppnd -h`:
```
ppnd 0.9.9 -- a prettier /proc/net/dev

Options:
    -l, --show-lo       show loopback interface in list
                        (default: hide loopback)
    -s, --scale SCALE   scaling factor for byte count
                        (default: dyn10)
    -p, --precision PRECISION
                        precision of scaled byte count (0-8)
                        (default: 3)
    -r, --repeat COUNT  query /proc/net/dev COUNT times (1-60)
                        (default: 1)
    -d, --delay SECONDS delay between queries in SECONDS (1-60)
                        (default: 5)
    -h, --help          show this help

Valid parameters for SCALE are:
    
'raw' = raw byte count
'dyn10' = dynamic power-of-10 scaling (KB, MB, GB, etc.)
'dyn2' = dynamic power-of-2 scaling (KiB, MiB, GiB, etc.)
'kb', 'mb', 'gb', 'tb', or 'pb' = fixed scaling (power-of-10)
'kib', 'mib', 'gib', 'tib' or 'pib' = fixed scaling (power-of-2)
```
And, here is some sample output from the application itself:
```
jack@asus - 11:53:21 - [ ~ ]
1-0-2099 $> ppnd -l -s mib -p 5

lo:

RX Bytes      7.07745 MiB  |  TX Bytes      7.07745 MiB
RX Packets          89643  |  TX Packets          89643
RX Errors               0  |  TX Errors               0
RX Drops                0  |  TX Drops                0
RX FIFO                 0  |  TX FIFO                 0
RX Compressed           0  |  TX Compressed           0
RX Frames               0  |  TX Collisions           0
RX Multicast            0  |  TX Carrier              0

enp35s0:

RX Bytes      6161.57450 MiB  |  TX Bytes      6021.41109 MiB
RX Packets           5595209  |  TX Packets           5823911
RX Errors                  0  |  TX Errors                  0
RX Drops                 171  |  TX Drops                   0
RX FIFO                    0  |  TX FIFO                    0
RX Compressed              0  |  TX Compressed              0
RX Frames                  0  |  TX Collisions              0
RX Multicast           49851  |  TX Carrier                 0
```
I ported this from a Ruby script I wrote and have been using for ages. For me, this is primarily a Rust learning exercise. Hence, I would describe the feature set of this application as complete. Any future changes will be behind the scenes (and invisible to the end user).

If you find it useful, by all means use it.

If you want to change it, by all means fork it and change it. Attribution would be nice, but is not required.
