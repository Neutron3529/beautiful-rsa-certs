1. `cargo test` # ensure that the program could compile. there exists a simple test that could end up very fast.
2. `cargo build --release` # you would better add this --release flag, since it really takes a lot of time finding the corresponding safe prime.
3. (Optional, helpful in security but take a huge amount of time) if you want to get a ssh key quickly, edit the `main.rs` file, change `false,false` into `true,false`, which may increase the safety that only 1 prime is not safe prime.
4. edit the String `"////Do+you+really+think+this+is+a+vaild+rsa+public+key//Actually+it+is+just+a+toy+which+happened+to+work+with+openssh////"` to whatever you want. the last character may be modified, keep it in mind.
5. run the program, then you could have a good sleep if you do modify the parameter to `true,true`. I have no idea to know how long the program would take. Mine is still running.
6. the program will end up with two block, like
```
-----BEGIN PUBLIC KEY-----
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEArBnhT6AGlFgSMfikvscg
DgtKTlHIjnqb/TCzzm8xGGRCZnfvoM45fAcwcMCPovEmb8BN19W/a0JEUYcMadnU
3jL0Vd4UrkCYXeM6k2JAHBE6AFcESBGjlKM0XJcvibKaOqhiFyCFnUgNLaAhxWAt
gg1qfIdSYSTfioglfjUmUoM9a1GZg02qnNPRNAqGHkqu2WgFqwIB1moT5eKhFk5k
W3S1eJq7RwORtbiEwa769L9DB1C6LH/BEWCrSz2aL19Aw9Pr5CabLPNXCCKxOZkN
FDzxTG9Rozqhs0PIyH7pYgmwQSiFUnUYbwp40hjuiCXtMIt40sgA98e+80p3fzNW
X/Hze1M0gAKh2PqIqxKiWs7wv0B5p6aOOn8tn51w65tLX90tGCDaZTQ1pYkyEFwX
C1VGBlLUavQZM4PMrrnweb7YhNAFRyL0Tr+tbf151XdVPFx7cz24jkgxEOVpKR0p
xvon9if5uDYZ+NAa8YIaa6ASg/jxTVjfBryICP2bNY5O/2pADqvD5mhZlIFU6hXl
egYyfyI5rdynYNK9FTDS2TIbCWJBdKtzIa9+Gv0SneoL1N8ofdO1HxsACAQ4Arh4
W10+QfjMxWDAjNnSjl9+h4rtjcp+n98CqyZtZ41s61q/TPBMMiWl++ThisIsARea
llyBeautifulEnding+++P8CAwEAAQ==
-----END PUBLIC KEY-----


-----BEGIN PRIVATE KEY-----
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
-----END PRIVATE KEY-----
```
   and you may see the pattern occurs in the public key.

7. Actually this pattern do not occurs in the format of ssh-rsa. If you want the same pattern shown in openssh format, like
```
ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCPEm1uItP2GavhajrngzefLb4fdKeoaX1ijdT1UJ1X9As8793hzChETUo5GOGIdlM+9FtxsFpkexT5VL7/Ni50rG7/PNwxPUBHZwR9NDrCOinI3uQaVo2cJsuNy3Jtc9/UHLS4blxVEVtboSE4BxzPT65Dfb0jhwnGl6UCteGUnEiwDXKVAu3cYaffHZX2hF3y8+q+OjwfMVb5kk1Qc4zd2iRYQTNDRAOv/b5YlQz8U6MsBO7l6WDPUgqLsi/6T19V2ohR4QrauEoSdCITPK2cQInxdcfdpSHiRcCZ40XgKRC4SpAcIsytzaXZjtX/genEg7Nefa0d6KZ20T0MUhfq7bPd6S0G7/p8YO2n02Nb2DsKmFJZ+fWdxpTyFBMowHaa5UwSmiKX1GXv2EHCIM+CWvRhnkdrQ3PT9aHRBMCFODPWwwrKmnK0/6cn3BzqRxZUpDMPFQTGqZ9DNhsmnHyKdmc6B0sLaaF09e01L5j7spWmEYj20KTHLZx4rCjUjuiEyxXB7+uiZxaUo5wRE45dNEz2flAK71n8n5UgtWd25WMMdpsm////Do+you+really+think+this+is+a+vaild+rsa+public+key//Actually+it+is+just+a+toy+which+happened+to+work+with+openssh///Pw==
```
   you should firstly change the suffix of the string to choose the correct align.

8. If you're quite sure you got the correct private key, paste it into a file (e.g., `test.rsa`), then execute
```
chmod 600 test.rsa ; ssh-keygen -y -f test.rsa
```
   the result may shown on your screen.

9. Enjoy the toy!
