beautiful-rsa-certs
---

This is a toy that is possible to generating beautiful but unsafe RSA key with arbitary pattern near its suffix.
Repeat, This is only a toy, **SHOULD NOT** use for keep *really secret things*.

Actually it could only provide 1 safe prime (rather than expected 2 safe primes) due to the inefficiency. if you edited the parameter to ask for 2 safe primes, it may take several hours, even several days to finish generating the key-pairs.

Due to unfamilar with the storage structure of RSA files, I put an extra suffix in the file to ensure we could control the correct alignment that would not encoding our designed keys into a really mass.

--as you can see, you should manually edit the suffix and the string.

This is now no more than a funny toy.

Do **NOT** use it in the real world unless you **DO** spend several days for two safe primes.
