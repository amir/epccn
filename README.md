# epccn
Goes through list of open network sockets of a Linux machine and if the remote address's port is `443` or `8443` tries to download the peer certificate and extract Common Name from Subject Name.

## Sample Output
```λ ~  epccn
23.207.185.232:443      Established     Ok("i1.social.s-msft.com")
157.56.75.164:443       Established     Ok("social.msdn.microsoft.com")
52.32.209.197:443       Established     Ok("push.services.mozilla.com")
23.207.185.232:443      Established     Ok("i1.social.s-msft.com")
192.30.253.124:443      Established     Ok("*.github.com")
192.30.253.125:443      Established     Ok("*.github.com")
23.207.185.232:443      Established     Ok("i1.social.s-msft.com")
34.211.99.53:443        TimeWait        Ok("*.services.mozilla.com")
157.56.75.164:443       Established     Ok("social.msdn.microsoft.com")
176.34.155.23:443       Established     Ok("*.duckduckgo.com")
192.30.253.112:443      Established     Ok("github.com")
66.111.4.147:443        Established     Ok("www.fastmail.com")
23.207.185.232:443      Established     Ok("i1.social.s-msft.com")
34.211.99.53:443        TimeWait        Ok("*.services.mozilla.com")
```
