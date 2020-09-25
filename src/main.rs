use blockchainlib::*;
fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0;32], 118318, "Genesis block".to_owned(), difficulty);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("Mined genesis block: {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());


    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty);
        block.mine();

        println!("{:?}", &block);
        last_hash = block.hash.clone();
        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
    }

    // blockchain.blocks[3].index = 6;
    blockchain.blocks[3].hash = last_hash;
    println!("Verify: {}", &blockchain.verify());
}
