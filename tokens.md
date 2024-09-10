# Tokens

Some notes on deployed tokens.

* [1Sat Market](https://1sat.market/)

## Points of interest

OIL deployment transaction, 7ab0b8c2c4cc67e2fc01317de1618e619765ca3b1ba43f637fbed3f44d7ab002, output 0, has a script that 
contains errors at the end. But it is still recognized by some parsers, as it is present on the market. It looks to me like
an error was made in publishing the script and its a miracle they were able to spend it. There's an OP_RETURN near the end
and before the error so it may have been possible.

## DAGON - BSV-21

* Deployment - [0da732c8b1a7e4a4292cc2d5bbb63d6394e5c39c95904b254473671da0adfbf8](https://whatsonchain.com/tx/0da732c8b1a7e4a4292cc2d5bbb63d6394e5c39c95904b254473671da0adfbf8)

## LOL - BSV-20

* Deployment - [8cc490114ee8013a741733fb9a51cb875a01f05ec5c4aeb5ec2e17ca31554e6e](https://whatsonchain.com/tx/8cc490114ee8013a741733fb9a51cb875a01f05ec5c4aeb5ec2e17ca31554e6e) 

## OIL - BSV-21

* Note: strange deployment script, see above.
* Deployment - [7ab0b8c2c4cc67e2fc01317de1618e619765ca3b1ba43f637fbed3f44d7ab002](https://whatsonchain.com/tx/7ab0b8c2c4cc67e2fc01317de1618e619765ca3b1ba43f637fbed3f44d7ab002)
* [Market](https://1sat.market/market/bsv21/7ab0b8c2c4cc67e2fc01317de1618e619765ca3b1ba43f637fbed3f44d7ab002_0)

## SATS - BSV-21

* Deployment - [1e97931fed2e8f8b6a4879917851a6370076f0c87bf538cc768a5c8f6e780d76](https://whatsonchain.com/tx/1e97931fed2e8f8b6a4879917851a6370076f0c87bf538cc768a5c8f6e780d76)

## SCALES - BSV-21

* Note: strange deployment script similar to OIL
* Deployment - [157bf3b7104c0badfa88fee9851d1b4cb601760828478f95dfdddbab6c5a238c](https://whatsonchain.com/tx/157bf3b7104c0badfa88fee9851d1b4cb601760828478f95dfdddbab6c5a238c)
