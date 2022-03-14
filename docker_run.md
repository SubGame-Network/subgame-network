# SubGame Network

## Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Download [newFork.zip](https://document.subgame.org/newFork.zip) and unzip get file `newFork.json`

Then run the following command to start a single node chain.

```bash
docker-compose up
```

> This command will firstly compile your code, and then start a network. You can
also replace the default command by appending your own. A few useful ones are as follow.

With this command, you should see something like this if your node is running successfully:

```bash
2021-03-16 10:56:51  Running in --dev mode, RPC CORS has been disabled.
2021-03-16 10:56:51  Substrate Node
2021-03-16 10:56:51  ✌️  version 3.0.0-8370ddd-x86_64-linux-gnu
2021-03-16 10:56:51  ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2021
2021-03-16 10:56:51  📋 Chain specification: Development
2021-03-16 10:56:51  🏷 Node name: few-size-5380
2021-03-16 10:56:51  👤 Role: AUTHORITY
2021-03-16 10:56:51  💾 Database: RocksDb at /tmp/substrateP1jD7H/chains/dev/db
2021-03-16 10:56:51  ⛓  Native runtime: subgame-100 (subgame-1.tx1.au1)
2021-03-16 10:56:51  🔨 Initializing Genesis block/state (state: 0x17df…04a0, header-hash: 0xc43b…ed16)
2021-03-16 10:56:51  👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2021-03-16 10:56:51  ⏱  Loaded block-time = 6000 milliseconds from genesis on first-launch
2021-03-16 10:56:51  Using default protocol ID "sup" because none is configured in the chain specs
2021-03-16 10:56:51  🏷 Local node identity is: 12D3KooWQdU84EJCqDr4aqfhb7dxXU2fzd6i2Rn1XdNtsiM5jvEC
2021-03-16 10:56:51  📦 Highest known block at #0
2021-03-16 10:56:51  〽️ Prometheus server started at 127.0.0.1:9615
2021-03-16 10:56:51  Listening for new connections on 127.0.0.1:9944.
2021-03-16 10:56:54  🙌 Starting consensus session on top of parent 0xc43b4514877d7dcfff2459cdfe609a96cf8e9b9723589635d7215de6bf00ed16
2021-03-16 10:56:54  🎁 Prepared block for proposing at 1 [hash: 0x255bcf44df92dd4ccaca15d92d4a3db9d276e42843e21ab0cc840e207b2649d6; parent_hash: 0xc43b…ed16; extrinsics (1): [0x02bf…2cbd]]
2021-03-16 10:56:54  🔖 Pre-sealed block for proposal at 1. Hash now 0x9c14d9caccc37f8142fc348d184fb4bd8a8bc217a8979493d7f46d4220775616, previously 0x255bcf44df92dd4ccaca15d92d4a3db9d276e42843e21ab0cc840e207b2649d6.
2021-03-16 10:56:54  ✨ Imported #1 (0x9c14…5616)
2021-03-16 10:56:54  🙌 Starting consensus session on top of parent 0x9c14d9caccc37f8142fc348d184fb4bd8a8bc217a8979493d7f46d4220775616
2021-03-16 10:56:54  🎁 Prepared block for proposing at 2 [hash: 0x6cd4bd9d2a531750c10610bdaa5af0075745b6612ffa3623c14d699250b4e732; parent_hash: 0x9c14…5616; extrinsics (1): [0x3cc8…b8d9]]
2021-03-16 10:56:54  🔖 Pre-sealed block for proposal at 2. Hash now 0x05bd3317b51d717163dfa8847369d7f697c6180868c29f02d0b7ff79c5bbde3f, previously 0x6cd4bd9d2a531750c10610bdaa5af0075745b6612ffa3623c14d699250b4e732.
2021-03-16 10:56:54  ✨ Imported #2 (0x05bd…de3f)
2021-03-16 10:56:56  💤 Idle (0 peers), best: #2 (0x05bd…de3f), finalized #0 (0xc43b…ed16), ⬇ 0 ⬆ 0
2021-03-16 10:57:00  🙌 Starting consensus session on top of parent 0x05bd3317b51d717163dfa8847369d7f697c6180868c29f02d0b7ff79c5bbde3f
2021-03-16 10:57:00  🎁 Prepared block for proposing at 3 [hash: 0xa6990964cf4f184edc08acd61c3c01ac8975abbba6d42f4eec3f9658097aec04; parent_hash: 0x05bd…de3f; extrinsics (1): [0xd6ed…86a5]]
2021-03-16 10:57:00  🔖 Pre-sealed block for proposal at 3. Hash now 0xbe07e322ca525e580a3703637db191c6df091b0242a411b88fa0c43ef0ac31f8, previously 0xa6990964cf4f184edc08acd61c3c01ac8975abbba6d42f4eec3f9658097aec04.
2021-03-16 10:57:00  ✨ Imported #3 (0xbe07…31f8)
2021-03-16 10:57:01  💤 Idle (0 peers), best: #3 (0xbe07…31f8), finalized #1 (0x9c14…5616), ⬇ 0 ⬆ 0
```

If the number after `finalized:` is increasing, your blockchain is producing new blocks and reaching
consensus about the state they describe!

> While not critical now, please do read all the startup logs for your node, as they help inform you
> of key configuration information as you continue to learn and move past these first basic tutorials.

## Start the Front-End Template

To interact with your local node, we will use
[the Substrate Developer Hub Front-End Template](https://github.com/substrate-developer-hub/substrate-front-end-template),
which is a collection of UI components that have been designed with common use cases in mind.

> Be sure to use the correct version of the template for the version of substrate you are using
> as [major versions](https://semver.org/) are _not_ expected to be interoperable!

You already installed the Front-End Template; let's launch it by executing the following command
in the root directory of the Front-End Template:

```bash
# Make sure to run this command in the root directory of the Front-End Template
yarn start
```

## Interact

Once the Front-End Template is running and loaded in your browser at
[http://localhost:8000/](http://localhost:8000/), take a moment to explore its components. At the
top, you will find lots of helpful information about the chain to which you're connected as well as
an account selector that will let you control the account you use to perform on-chain operations.

## Learn More

Learn more about how the Substrate works

[Substrate Developer Hub](https://substrate.dev/)
