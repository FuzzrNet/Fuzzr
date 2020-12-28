# Fuzzr Project Milestones

Periodic updates to this document will be made to better communicate and record existing progress, and in addition, communicate project intentions in the future.

## 2020

> State of the project as of 2020

- Project kicked off December 2nd, 2020!
- Demo-quality desktop-native iced UI
    - Very good progress here, will be very useful as a springboard for future work.
- Integrated ipfs-embed-rs
    - Although quite capable, it's not ideal, simply because the traffic might not be as compatible or capable as mainline go-ipfs. That said, I'm not 100% ready to give up on it yet.
- Initial demo (even if it has some bugs)
    - The demo works in terms of being capable of adding image data to ipfs-embed-rs, but it's not yet capable of pulling it back out.
- An initial implementation of a combined asynchronous task management backend has been outlined in a draft PR.
- A Discord community server has been created!
- The project also has some nice, shiny documentation.

Fuzzr kickoff has been exciting, promising, and encouraging, thanks to contributions from kn0wmad (Dave), NukeManDan (Dan), and cryptoquick (Hunter).

## 2021

> Goals for 2021 and beyond

### API Integrations

- First, let's describe our terminology for turnkey integrations
    - First-party: Local, self-hosted, owned and operated by the individuals themselves
    - Second-party: Businesses that officially support and encourage adoptions from individual owners
    - Third-party: Experimental, community-contributed integrations for third-party APIs on behalf of individual operators
- First-party API integrations:
    - [ ] go-ipfs: More features, more standard, better integrates with the broader IPFS network
    - [ ] Monero daemon - Manages monero address generation and wallet
    - [ ] Monero miner, xmrig - CPU miner to provide monetary contributions to network contributors
    - [ ] Grin daemon - Manages grin addresses and wallet
    - [ ] Grin miner - Mines with GPU to provide PoW voting hashes
- These first-party integrations will take up lots of space and resources and UI
- Second-party API integrations:
    - [ ] Monero mining pools
    - [ ] Grin mining pools
- Specifications and request for community contributions for third-party API integrations.
    - These will have experimental status until second-party support is confirmed.
    - This will help encourage adoption of Fuzzr if it can be used to upload to and easily monetize with a variety of services. It also provides a sort of selective pressure to discourage censorship and deplatforming. Communities should be able to, for one, collectively decide that's not okay. Second, it helps diversify income streams. Finally, if a third-party platform disenfranchises enough individuals, their customers will be more likely to jump ship to follow their favorite content owners.

### Multiuploading

- A definition for two sides of the market:
    - Contributors
        - This is a better term, rather than users, since the term user can imply a lack of compensation of any kind for content owners. Further, it's possible, and encouraged, to be both.
        - Furthermore, the term users also implies a sort of detachment from support that can lead to exploitation of account holders for things that violate privacy such as marketing data.
        - "Creators" is an overused term. Contributions can take many forms, including monetary support, and even simply boosting contributions of others through sharing it within their networks, and contributing hashes towards votes and monetization.
    - Content owners
        - An emphasis should be placed on ownership of content. This is a way to discourage posting content owned by others. P2P business models should be treated the same as any other business.
            - Yes, this includes pesky things like accounting, paying taxes, creating corporations. We do want to make this as simple as possible, and provide plenty of good information to give owners
        - Ownership is encouraged. Copying others' content without permission is not exercizing sufficient individual responsibility, a value any responsible business owner should take to heart.
        - All applicable laws should be followed to the best of an owner's ability.
            - Even mentally, conceptually, in one's own mind, the concept of allowing the government to hold back the success of any business is giving the government too much power. Laws should be followed, but dependence upon centralized systems should be considered market opportunities for disruption. P2P business should eventually be substantially cheaper. Businesses are allowed to charge less for their services. This is essential for preserving existing market dynamics. Imposing a "minimum price" for a good or service can really only be done under a form of command economy, and that is simply a law. Laws can change, they aren't ever-present truths.

### Voting

- CPU mining with Monero can be profitable, but it's not a good way to provide a sort of P2P "upvote" or "like" dynamic.
- GPU hashes could be a good way to provide an additional layer that can include monetization, but that's not the only purpose. Having both encourages adoption of alternative cryptocurrencies like Grin that have merit, but aren't necessarily as profitable to mine currently. The intention is to provide a finite number of hashes for a given public key. Also, these can stack, allowing for individuals to provide multiple votes, but PoW disincentivizes extraordinary amounts of "ballot stuffing", as can occur in voting systems backed by only an email account or phone number, which do not provide sufficient Sybili resistance.

### Pages

- IPNS pages
- Owned and modified by individual contributors
- Can be forked, similar to reposting. Includes attribution to the original, but also, allows for editing, remixing, corrections, and even modifying meme formats.

### User Interface

- A more formal specification of UI goals and guidelines for implementations is needed.
- A rough outline of design philosophy, however:
    - Monochrome UI
        - Simpler, clean lines, allows for simple theme customization and sharing
        - Uses less power if using primary colors on OLED or MicroLED displays (red, green, or blue interfaces use 1/3 the power of a white pixel). Cyan, magenta, and yellow also use 2/3 the power.
    - Text should be used instead of icons, even for interactive buttons, with emoji being an exception.
    - The spec should be able to be extended, and optional for contributers, similar to CommonMark extensions. For example, if icons are deemed absolutely necessary for the UI, well, provide a text fallback (similar to an alt tag, like in Markdown and HTML image tags) that describes the button.
        - Icons complicate internationalization and localization.
        - They're also just unnecessarily complex, and visually busy.
    - Visual cleanliness and standardization
        - Content should be capable of formatting by the viewer, as per their preference.
        - Content should be considered upon the merit of the content alone, and not simply the presentation.
        - Freeform HTML, CSS, and JS formats allow for too much complexity to enter the viewer's systems, causing potential security flaws, or at least unideal UX, such as as ads, tracking, popup modals, and cookie banners.

### Web Server

- [ ] A warp server should be included in every Fuzzr client. This allows HTTP sharing across local LAN, and also allows the program to be configured and run on web servers easily. This encourages self-ownership of infrastructure. A self-owned Fuzzr web node should be capable of being scoped to content using specified public keys under a specific set of topics.
- [ ] This helps publish content in a self-owned way, while still providing an experience to visitors that analogous to, say, a blog on the centralized web.

### Data Packets

- [ ] QR codes could serve as a form of secure "data packet" to communicate things such as addresses to articles, accounts, and even facilitate decentralized ICE (Interactive Connectivity Establishment) like WebRTC's turn, using a series of swaps to establish a P2P connection between different devices, without requiring a middleman discovery server. Would have to be done in-person, or at least, out of band.
- [ ] QR codes should be displayed as unicode blocks, for copying and pasting, and QR readers simply squash the image vertically after an axial alignment transformation. Should work a variety of fonts, so axial alignment and detection of QR bit size will likely both have to take place.
- [ ] Dedicated desktop reader units should also be recommended, to easily read optical data packets
- [ ] An additional form of data packet communication could also be like a series of tones, like how old telephone modems operated, especially for the short period in which two-way ICE communication must take place. This only would work in quiet environments, of course. Both optical and audible methods should be available.

### Streaming Media

- Investigation and design around streaming media formats and techniques is ongoing, from an operational, monetary, and consumer perspective.
