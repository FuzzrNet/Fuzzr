# intents / goals

## general

- no launcher, actually, the app will open instantly always
- asynchronous processing of anything that takes a while, and nothing blocks user interaction as it's occurring.
- state (scrolling, content in text boxes, etc.) is fully independent of all other state, and is never "reset" without the user explicitly clearing or resetting that state.

- automatic format detection from drag and drop.
- integrated media players and encoders
  - [gstreamer](https://crates.io/keywords/gstreamer) looks like our best bet, remarkably enough.
- markdown for text contributions for now (no wysiwyg)
- mobile support

## viewing

- dashboard
- feed
- content type filter
- tag filter
- sort controls
- flexible view formatting
  - lists - as many columns as you like, with independent scroll state, or linked
  - grids - since iced doesn't have grids yet, we'll do a more pinterest-style masonry view, optional, for all content feeds. really just a lot of lists.

## terms

- #hashbank / hash balance / hashrate
- fuzzfeed

## identities / aliases

- this gives you bubbles, and that's okay
- separation of account (node) and identity

## intro

- welcome to fuzzr, presently you are a part of the network, but are currently anonymous and unknown to other participants
- you can use the majority of fuzzr's features in this way, such as viewing, purchasing, subscribing, and contributing content,
- but you can also create as many identities as you like.
- by adding tags to your identity you are essentially creating a social bubble.
- the advantage of this being, you completely control this bubble, allowing more honest expression.
- these identities are independent of each other to the extent that you want them to be.
- you can subscribe to contributors **or** tags.
- gold star for, this content was published earlier than the timestamp in the blockchain.
- download for free, but stream for hashes.

## MunShits

  - Every client is a WireGuard client, creating a sort of sort-of meshnet
    - Pay some hashes for routing through certain regions/jurisdictions
  - Ability to create your own section of the network (like a webpage), simply and graphically, and publish to it
