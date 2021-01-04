# P2P Data Model

Fuzzr uses IPFS, which is essentially software that connects to a swarm of peers capable of providing and asking for data.

Fuzzr also uses a Monero miner called XMRig. We configure this mining software to mine with only your CPU, because GPU mining for Monero is very computationally inefficient (by design). CPU mining uses less electricity, and is more versatile.




## Peers

Consider your Monero hashrate as your currency for asking peers for bytes. If your miner can hash 1 KH/s, and the peer sets their ratio to 1000:1, you can download from a single peer at 1MB/s


We use the IPFS Bitswap Ledger in a few ways:

- Check if we owe peers bytes
- Check if a peer owes us a significant number of bytes

## Providing



## Bootstrapping

TODO: Bootstrap Nodes help provide peers to connect to
