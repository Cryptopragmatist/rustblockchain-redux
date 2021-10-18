use rustchain::*; //import lib to access the modules and their functions


fn main() {
    let block = Block::new(0,0, vec![0;32],0 ,"Genesis block".to_owned()); //generate the genesis block with 0's as it is the first

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}",&h);
}
