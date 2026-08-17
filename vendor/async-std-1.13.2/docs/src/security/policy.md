# Policy

Safety is one of the core principles of what we do, and to that end, we would like to ensure that async-std has a secure implementation. Thank you for taking the time to responsibly disclose any issues you find.

All security bugs in async-std distribution should be reported by email to florian.gilcher@ferrous-systems.com. This list is delivered to a small security team. Your email will be acknowledged within 24 hours, and you’ll receive a more detailed response to your email within 48 hours indicating the next steps in handling your report. If you would like, you can encrypt your report using our public key. This key is also On MIT’s keyserver and reproduced below.

Be sure to use a descriptive subject line to avoid having your report be missed. After the initial reply to your report, the security team will endeavor to keep you informed of the progress being made towards a fix and full announcement. As recommended by [RFPolicy][rf-policy], these updates will be sent at least every five days. In reality, this is more likely to be every 24-48 hours.

If you have not received a reply to your email within 48 hours, or have not heard from the security team for the past five days, there are a few steps you can take (in order):

* Post on our Community forums

Please note that the discussion forums are public areas. When escalating in these venues, please do not discuss your issue. Simply say that you’re trying to get a hold of someone from the security team.

[rf-policy]: https://en.wikipedia.org/wiki/RFPolicy

## Disclosure policy

The async-std project has a 5 step disclosure process.

* The security report is received and is assigned a primary handler. This person will coordinate the fix and release process.
* The problem is confirmed and a list of all affected versions is determined.
* Code is audited to find any potential similar problems.
* Fixes are prepared for all releases which are still under maintenance. These fixes are not committed to the public repository but rather held locally pending the announcement.
* On the embargo date, the changes are pushed to the public repository and new builds are deployed to crates.io. Within 6 hours, a copy of the advisory will be published on the the async.rs blog.

This process can take some time, especially when coordination is required with maintainers of other projects. Every effort will be made to handle the bug in as timely a manner as possible, however it's important that we follow the release process above to ensure that the disclosure is handled in a consistent manner.

## Credits

This policy is adapted from the [Rust project](https://www.rust-lang.org/policies/security) security policy.

## PGP Key

```text
-----BEGIN PGP PUBLIC KEY BLOCK-----

mQENBF1Wu/ABCADJaGt4HwSlqKB9BGHWYKZj/6mTMbmc29vsEOcCSQKo6myCf9zc
sasWAttep4FAUDX+MJhVbBTSq9M1YVxp33Qh5AF0t9SnJZnbI+BZuGawcHDL01xE
bE+8bcA2+szeTTUZCeWwsaoTd/2qmQKvpUCBQp7uBs/ITO/I2q7+xCGXaOHZwUKc
H8SUBLd35nYFtjXAeejoZVkqG2qEjrc9bkZAwxFXi7Fw94QdkNLaCjNfKxZON/qP
A3WOpyWPr3ERk5C5prjEAvrW8kdqpTRjdmzQjsr8UEXb5GGEOo93N4OLZVQ2mXt9
dfn++GOnOk7sTxvfiDH8Ru5o4zCtKgO+r5/LABEBAAG0UkZsb3JpYW4gR2lsY2hl
ciAoU2VjdXJpdHkgY29udGFjdCBhc3luYy1zdGQpIDxmbG9yaWFuLmdpbGNoZXJA
ZmVycm91cy1zeXN0ZW1zLmNvbT6JATgEEwECACIFAl1Wu/ACGwMGCwkIBwMCBhUI
AgkKCwQWAgMBAh4BAheAAAoJEACXY97PwLtSc0AH/18yvrElVOkG0ADWX7l+JKHH
nMQtYj0Auop8d6TuKBbpwtYhwELrQoITDMV7f2XEnchNsvYxAyBZhIISmXeJboE1
KzZD1O+4QPXRcXhj+QNsKQ680mrgZXgAI2Y4ptIW9Vyw3jiHu/ZVopvDAt4li+up
3fRJGPAvGu+tclpJmA+Xam23cDj89M7/wHHgKIyT59WgFwyCgibL+NHKwg2Unzou
9uyZQnq6hf62sQTWEZIAr9BQpKmluplNIJHDeECWzZoE9ucE2ZXsq5pq9qojsAMK
yRdaFdpBcD/AxtrTKFeXGS7X7LqaljY/IFBEdJOqVNWpqSLjGWqjSLIEsc1AB0K5
AQ0EXVa78AEIAJMxBOEEW+2c3CcjFuUfcRsoBsFH3Vk+GwCbjIpNHq/eAvS1yy2L
u10U5CcT5Xb6be3AeCYv00ZHVbEi6VwoauVCSX8qDjhVzQxvNLgQ1SduobjyF6t8
3M/wTija6NvMKszyw1l2oHepxSMLej1m49DyCDFNiZm5rjQcYnFT4J71syxViqHF
v2fWCheTrHP3wfBAt5zyDet7IZd/EhYAK6xXEwr9nBPjfbaVexm2B8K6hOPNj0Bp
OKm4rcOj7JYlcxrwhMvNnwEue7MqH1oXAsoaC1BW+qs4acp/hHpesweL6Rcg1pED
OJUQd3UvRsqRK0EsorDu0oj5wt6Qp3ZEbPMAEQEAAYkBHwQYAQIACQUCXVa78AIb
DAAKCRAAl2Pez8C7Uv8bB/9scRm2wvzHLbFtcEHaHvlKO1yYfSVqKqJzIKHc7pM2
+szM8JVRTxAbzK5Xih9SB5xlekixxO2UCJI5DkJ/ir/RCcg+/CAQ8iLm2UcYAgJD
TocKiR5gjNAvUDI4tMrDLLdF+7+RCQGc7HBSxFiNBJVGAztGVh1+cQ0zaCX6Tt33
1EQtyRcPID0m6+ip5tCJN0dILC0YcwzXGrSgjB03JqItIyJEucdQz6UB84TIAGku
JJl4tktgD9T7Rb5uzRhHCSbLy89DQVvCcKD4B94ffuDW3HO8n8utDusOiZuG4BUf
WdFy6/gTLNiFbTzkq1BBJQMN1nBwGs1sn63RRgjumZ1N
=dIcF
-----END PGP PUBLIC KEY BLOCK-----
```
