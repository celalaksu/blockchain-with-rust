use std::vec;

use blockchainlib::*;

fn main() {

    // VIDEO 5
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    
    // let mut genesis_block = Block::new(0, now(),vec![0; 32], vec![
    //     Transaction {
    //         inputs: vec![],
    //         outputs: vec![
    //             transaction::Output {
    //                 to_addr: "Alice".to_owned(),
    //                 value: 50,
    //             },
    //             transaction::Output {
    //                 to_addr:"Bob".to_owned(),
    //                 value: 7,
    //             },
    //         ],
    //     }
    // ], difficulty);

    // Invalid transaction ( panics and give InvalidCoinBaseTransaction error)
    // let mut genesis_block = Block::new(0, now(),vec![0; 32], vec![
    //     Transaction {
    //         inputs: vec![
    //             transaction::Output {
    //                 to_addr: "Chris".to_owned(),
    //                 value: 50,
    //             },
    //         ],
    //         outputs: vec![
    //             transaction::Output {
    //                 to_addr: "Alice".to_owned(),
    //                 value: 50,
    //             },
    //             transaction::Output {
    //                 to_addr:"Bob".to_owned(),
    //                 value: 7,
    //             },
    //         ],
    //     }
    // ], difficulty);

    let mut genesis_block = Block::new(0, now(),vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr:"Bob".to_owned(),
                    value: 7,
                },
            ],
        }
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block {:?}",&genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block!");

    // Adding another block
    let mut block = Block::new(1, now(),last_hash, vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Celal".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),

                // Try with Invalid Input - Next transaction doesn't exist - gives invalidinput eror.
                // transaction::Output {
                //     to_addr: "Nobody".to_owned(),
                //     value: 363896,
                // },
                
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 36,
                    // sending more value than alice has. InsufficientInputValue
                    //value: 360,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        }
    ], difficulty);

    block.mine();

    println!("Mined  block {:?}",&block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add  block!");

    // VIDEO 5 END ( NO MORE CODE IN VIDEO 4 )

    // VIDEO 3
    // let difficulty = 0x000fffffffffffffffffffffffffffff;

    // let mut block = Block::new(0, now(), vec![0; 32], 
    //      0, "Genesis block".to_owned(), difficulty);

    // block.mine();
    // println!("Mined genesis block {:?}",&block);

    // let mut last_hash = block.hash.clone();

    // let mut blockchain = Blockchain {
    //     blocks: vec![block],
    // };

    // for i in 1..=10 {
    //     let mut block = Block::new(i, now(), last_hash, 
    //         0, "Another block".to_owned(), difficulty);
   
    //    block.mine();
    //    println!("Mined block {:?}",&block);

    //    last_hash = block.hash.clone();

    //    blockchain.blocks.push(block);

    //    println!("Verify: {} ", &blockchain.verify());
    // }

    // // to test verify for false return
    // // blockchain.blocks[3].index = 4;
    // //blockchain.blocks[3].hash[8] += 1;
    // //blockchain.blocks[3].payload = "Nope".to_owned();
    // blockchain.blocks[3].prev_block_hash[18] = 8;
    // println!("Verify: {} ", &blockchain.verify());

    // END VIDEO 3

    

    // VIDEO 2
    // let mut block = Block::new(0, 0, vec![0; 32], 
    //     0, "Genesis block".to_owned(), 0x00ffffffffffffffffffffffffffffff);

    // let mut block = Block::new(0, 0, vec![0; 32], 
    //     0, "Genesis block".to_owned(), 0x000fffffffffffffffffffffffffffff);

    // let mut block = Block::new(0, 0, vec![0; 32], 
    //         0, "Genesis block".to_owned(), 0x0008ffffffffffffffffffffffffffff);
    
    // to see time to find nonce, use ; time cargo run

    // let mut block = Block::new(0, 0, vec![0; 32], 
    //     0, "Genesis block".to_owned(), 0x000000ffffffffffffffffffffffffff);
    // // for this nonce is 11250013. When change nonce to 11250013, it will give same hash

    // block.hash = block.hash();

    // println!("{:?}",  &block);

    // block.mine();

    // println!("{:?}",  &block);
    // END video 2

    // - from first video
    // let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned());
    
    // println!("{:?}",  &block);

    // // hash fonksiyonu çağrılınca nesne hashable traite gönderilmektedir.
    // // hash fonksiyonu nesnenin bytes özelliği ile çalışmaktadır.
    // // bytes fonksiyonu block.rs de hashable Block için implement işleminde tanımlanmıştır.
    // // dolayısıyla bytes fonksiyonu block nesnesi içinde mevcuttur. yani otomatik olarak
    // // hash fonksiyonuna gönderilmektedir.
    // let h = block.hash();

    // println!("{:?}", &h);

    // block.hash = h;

    // println!("{:?}",  &block);
    // END - from first video 
}
