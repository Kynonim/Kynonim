## CONFIG ROUTER
```bash
enable
configure terminal
interface FastEthernet0/0
ip address 192.168.1.1 255.255.255.0
no shutdown
exit
interface FastEthernet0/1
ip address 192.168.2.1 255.255.255.0
no shutdown
exit
```
