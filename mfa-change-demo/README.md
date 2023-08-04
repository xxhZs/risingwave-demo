# recwave

feature store dome

## Installation

Run it in local.

1. Start kafka and risingwave. And create source and mv in risingwave. can run this script

./run.sh

we can change recwave-start.sql and server/src/serving to add new demands or modift it.

2. Start feature store serve demo. It can accept data writing to Kafka and provide feature query services

cd server
cargo run

3. Run data simulator. It can generate simulated data and simulate user queries.

cd simulator
cargo run




