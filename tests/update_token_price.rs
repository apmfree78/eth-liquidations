use abi::Bytes;
use bigdecimal::BigDecimal;
use eth_liquadation::data::token_data_hash::{
    get_token_data, save_btc_as_token, save_erc20_tokens_from_static_data,
};
use eth_liquadation::data::token_price_hash::{generate_token_price_hash, get_saved_token_price};
use eth_liquadation::mempool::decode_new_price::get_chainlink_price_from_transmit_tx;
use eth_liquadation::mempool::update_token_price::update_token_price_for_;
use ethers::prelude::*;
use serial_test::serial;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
#[serial]
async fn test_decoding_token_price_for_uni_token() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    println!("save tokens");
    save_erc20_tokens_from_static_data(&client).await?;

    let token_data = get_token_data().await?;
    let uni_token = token_data.get("UNI").unwrap();

    println!("generating token prices");
    generate_token_price_hash(&client).await?;

    println!("decoding price");
    // data for unitoken price update
    let data: Bytes = b"\xc9\x80u9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\x80\0\x01\x01\x01\0\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\xe0\0\0\0\0\0\0\0\0\0\0\0&\x07\x8b\x924\x95v\x16Y\xc5\xbdE\x03X\xfc\xa0\0\0\x14\xb4\x06\x02\x06\x0f\x10\n\x0e\t\x11\x05\x03\x12\r\x04\x08\0\x0b\x01\x0c\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#T#\xc4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#T\xa2\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#T\xa2\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#T\xa2\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#T\xb2D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#T\xf7\xb8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#U&\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#V\x8d\xf8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#V\x8d\xf8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#V\x8d\xf8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#V\x8d\xf8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#V\xbc\xd8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#W\xe9\xa0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#X\x02n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#X\x02n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#[q\xf6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#[q\xf6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#]M\xd6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0#]M\xd6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07@;\xdc\xe1\xaf\xea \xcfj2\xea\xd6q)k\xbavs\xe7Bq>#\xd3\xaf\xbd,\x14U\xd7.*`w=It\xfe\xff\xeb\x8e\xfc\xf7\xac\x1dOF\x98\xfei\x19]\xcd\xb4/\xf8l\xbb>\x1e\xfa\xacv\xb9\x86m\x8d\xcb\xba\xe5\xdd>\xc9>\x07\xd1\x86H\"~6\x05Z3\x8d\x98Ce\xb0\x9d\x87\xc6m.\xe2\x0217\xdc\x1e@>\xeft\r\xe7\xa4\x11\xcc\xd8\xff\x19k\xc1\xcd\xb7\xef\x1d\0\xbfU\xaaA\x8f\x06\x7f*\xd9\xe5\x95\x85[[\x05\x91o#\xd2\x0efxW\r\x08\x9a\xeb\xb7\x01\x18\x0f\x91M\x03\xa7\xeaF\xd6\x11\xe3\xec\xef\xbe\xd6\xb5~\xcb\x98\xf0\x06\x92\xd4\xea\xa5\xcd\xaa\xa69S\x9b0\xb3\x9bh=\x94\xb5Xb\xcfb\n\x9ar\xb2D\x9bO\xd4\xf3\x16\xc3W~\xf8K\xb7Zq\x02\xa0\xd4K\xf2A\x92X!\xd5\xfd\xaa;\x90\xa1\xc5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07CD{\x89\x111!\xabZ\xa9\t\x12c[\xaa\x9f\x95Y.p\xb9\xb5\xa7Q\x8cr\x8c\xa3%\x92\xf9\xd0W\xbb\xa4k\x13\r\xeb\x96\x90\xff\xe8\x95\xdf\xb5z\xfc\x9dcQ\xf6U\xc7\x92_\xaa\x84\x8d\xcdz\xb9\xfb\x8c\x07\x81\xf9R\xcf\x90\xc7\xee\xeeV\xb4;\xd8\xff\xb5\xcf\x9a\x1eJ1\xbf\x10cj\x84\x1a\xec \xcb\xc388A\xa7\xd1\xbb\xa2\xa3o\x95\x92j8~,\xca\x9fo\xb3\xccp\xd1\xf2\x82\x8d\xa4\xb4\xb7jY\xe6\xe4\x84c+\x10O \xec\xecR\xca\xba \xb8\x0eg\x11o\xc2\x87\xd3A\x17\xf7\x1d\xa9\xa1\xb6\xcd\xd1\x13X\xe6\xff\xd5\x10\x8e3\xb5\xbe\xb2\xa5\x9c\xf78\xe6z\xdd\xfb\xf9\xbf\x8bt\xd7\\\xeeB\x99\x9d\x89\xea\xf6\xed\x85\x83V\x0c=\x82\xa8d@\xd5\xd1c\xd2\x96\xb0\x05Y\xca0\x91\xf0\x81\xc1=w\xa2\xcd\xf0\xeb\xa9\xc3[\xd5x\xc5+".into();

    let new_price = get_chainlink_price_from_transmit_tx(&data, uni_token).await?;
    let expected_price = BigDecimal::from_str("5.92875")?;

    assert_eq!(new_price, expected_price);

    println!("updating token price");
    update_token_price_for_(uni_token, new_price, &client).await?;

    let new_price = get_saved_token_price(uni_token.address.clone()).await?;

    assert_eq!(new_price, expected_price);

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_decoding_token_price_for_weth_token() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    println!("save tokens");
    save_erc20_tokens_from_static_data(&client).await?;
    let token_data = get_token_data().await?;
    let weth_token = token_data.get("WETH").unwrap();

    println!("generating token prices");
    generate_token_price_hash(&client).await?;

    println!("decoding data");
    // data for unitoken price update
    let data:Bytes = b"\xc9\x80u9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\x80\0\0\x01\x01\0\x01\0\x01\x01\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04`\0\0\0\0\0\0\0\0\0\0\0\xc6\"\xbcED[\xfcM\xa0\xca\x80\xce,;\xf8\xca\0\0\x14\xc7\x05\x1b\x16\x02\x0c\x1c\x03\x04\x17\x12\x07\x0e\x05\x0b\x06\x08\x14\x1e\x01\0\x15\x10\x1d\x0f\r\t\x13\x18\x1a\x11\n\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa5m\x11\xa3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa5mf\xa3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa64@+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa8N\x17\xc2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa8N\x17\xc2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa8N\x17\xc2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa8N\x17\xc2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa8N\x17\xc2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa8N\x17\xc2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xa9\xd7\xc4U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa*D\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa*D\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa*D\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa*D\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa*D\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa\xa4V\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa\xa4V\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa\xa4V\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xaa\xa4V\xc0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xab\xc2~\x9c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xab\xc5`\x1c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xab\xc7i\xfc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xab\xcaS\xb0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xab\xd3<\xf0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xab\xd4*p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xac\x1cK\x9a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xac\x1cK\x9a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xac\x1cK\x9a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xac1\x11@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xac\xea\xc4P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0:\xad\xa4w`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0b\x16\xbd9\xadk\xedw\xa59\xc1\xac\xe9\x02][\xf9\x9e~<\t\x82\x1d\xf2*\x04\x03\xd24Z\xaed:\x94\xfa\r\xd1G\xdb\xb8\x1a#1=\xc9\x14\xab\xbfW\x0ct\xe3\x84\x8a\\\xaf\xa9\x13\x91\xc3\xb5\xc0s_\xc4\xb8\xea:\xc2\xb4L\x1dM\xd8\xb4A\xea4\xb9\xba-\xf0^\x86\xfe!3B1\\\x9c\xa6\x93\xed\x7f/\xa0\xa2O\xaa\xacIU\xc7'\x99\x0c\x8a2\x02\xdd\xf4d\xfa\xe4\x04\x8a\xd9\x94!\x81\xf84\x92:\xc2\xfdH^X*\xc4\xd3\xb7\xd8S\x11\0/\xaf\xc2O_\xeb\xb5\xcc\x02\xa07\x05\xd1\xb0\xbe\xb9J\xf3\x8b\xedE]\x8f\xd4\"i\xe6\x9b\xd4)@\x12+2\xef\xc3\x16T\x88(\x1f\xa5A<yU\xdcB\x18\x0f\x1azNi\xb7!/%\x89\xec\x1aD</:iK\xd0\x14=C\xe8k\xc6\xe5\xf9H\\\xd0UM\xc0\x8a\xb2\xbc\xf9\xa1i\xb7\x8f\xcd\x15p\xee\xaf\x9c8[\x90w\x03\xdb\xb7'\xbe\x92=\xa5\xc9\x0ftH\xe6\x18\xd8AY\xea\xa1\x02\xb1\xc5u\xa9\xac\x06\x01I\x15\xb8\x02<w\x0cU\xa6\xfd2\x08s\x9d\xac\x18\xa9!y\x83\x9eC\xab\xd6\x9d\xfa\xf0\x08X1\x9fo;\x14pH\x85\x0e\x17\xc9\xce\x01;\x8ez\x13\xd1\xb7\xbfr\x88\x17\xa0^\x99\x8fL\xc6\x8c\"\xd4\x97\x8c3m\xa2[\xe8A\x9d\xf2Vs\xe6\n\x83\t_\x96\xdb\xaf\xdbm\xcf\x06W$v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0b8\x04\xee\x91%U\\\x0f\xb0[AJ\xfc\"7\xe4[\x07\x0b\xe4(z\xff$\x87W\xa7\xfe{\x17m$Z\x83\xd9\x18l-\x19\xff_\xb3n\x93N\xf7jZ\xe5\xa1\x12\x91i\x18\x12 ?\x07\xd6\x80\x83P|\xb7,Q\xd7\xa9\x10Z%M:\x17\xeeSk\x9d\xb5\xf9\xd29\r\xc9\xd9\xef\xf2\xd7lx\x9d\xc1\x18\x88v\x1cu\x8b\xd22\\\xe7\xca\xfe\x0f}3\xc5\xad\x19\x9f\x99\xb1\x1eT@\x022\xa7\xcc\x07`\x14U\xef\xfbh\xb1cQ\xa1\x9bD\x10\xf79.\xe8\xf3-\x040.\xcd\xa9\x88\x1d\xd7\xf4*\xbf\xc8A\x04\xdc/h\x94-O7C`\xfa\t\xb6L\xc6\xb4\x8a\xe4\xf3\x827\x04\t\xac@\x89\x10s\x13\xef}O\xcf\x0b3\xc1\x1b\xdaY:\x0c;\xa7-\x0b\x91\xa3\x90\xd2C\xd6y\x02}\xc1\x84\xa0\xbc\x0c\xd5\x94C\xf4a\x90\xc1)\x0e\xc6k\xb1S\xb4s\xf26\xddL.\xc3B\xc8\xae\xae9!;\xc7\xc9\x1e\r\xa1-M\x0b\xd0\x82\xf3\xc7\xd4,\x19\xd73\x03\x10\xed\xa4\x1c\xb8I\xeakZ\x14\xd8\xa0a:mv&\xea\xf4\xd7\xa3\xd5\x03~\xc9\xd5n\xa3\xda#\x01\x8c\xb2\xdd\x84\xe3\xef\xbb\xa8\x88\xca\x96a\x03\x9f\t\xb0\xfa\xa8\xf1\xa3\x87\x15-\x99_\xcd\xd6\xb0N\xe4YV)]\x8e;\x0bLV?\xb5\xd8\x8a\x1c\xb0L\x11\xe4x3A\x03P\x01u`\xea\xef\xb5\xb6\xf5\x12=".into();
    let new_price = get_chainlink_price_from_transmit_tx(&data, weth_token).await?;

    let weth_expected_price = BigDecimal::from_str("2519.71")?;

    assert_eq!(new_price, weth_expected_price);

    println!("updating token price");
    update_token_price_for_(weth_token, new_price, &client).await?;

    let new_price = get_saved_token_price(weth_token.address.clone()).await?;

    assert_eq!(new_price, weth_expected_price);

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_decoding_token_price_for_btc_token() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Ethereum client connection and wallet
    const WS_URL: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(WS_URL).await?;
    let client = Arc::new(provider);
    // populate token state
    println!("save tokens");
    save_erc20_tokens_from_static_data(&client).await?;
    save_btc_as_token(&client).await?;
    // populate token state
    println!("save tokens");
    save_erc20_tokens_from_static_data(&client).await?;
    let token_data = get_token_data().await?;
    let btc_token = token_data.get("BTC").unwrap();

    println!("generating token prices");
    generate_token_price_hash(&client).await?;

    println!("decoding data");
    // data for unitoken price update
    let data: Bytes = b"\xc9\x80u9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\x80\0\x01\x01\0\x01\0\x01\0\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04`\0\0\0\0\0\0\0\0\0\0\0\xe6\x99\x01\xad\xaa\x19]\xc3A0Bh\x02\xaeY\xca\0\x02\xdb\xe5\x05\x17\x1e\x03\x14\x15\x05\x08\x0b\x0c\x16\x13\0\x11\t\n\x07\r\x01\x1b\x19\x0e\x04\x12\x1d\x0f\x18\x10\x1c\x06\x02\x1a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05]\xfd\x02\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05]\xfd\x02\xf2x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05]\xfdV\x9f\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05]\xfe\xf2b\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\0\xe3\xe5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x02\x11:d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x19\xd2\xa5`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x19\xd2\xa5`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x19\xd2\xa5`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x1d\xfa/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x1d\xfa/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x1d\xfa/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x1d\xfa/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\x1d\xfa/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^)`\x17.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^)`\x17.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^)`\x17.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^)`\x17.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^)`\x17.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^>\xb4\xc8\xe0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^>\xb4\xc8\xe0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^@\x18\xeb\xe0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^@\xa5'\x0c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^@\xa5'\x0c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05^\xad[0;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0b(S\x9cE\xca\xb4\xeeN`rT/vN\x99\xa2\xe3\x8c\x0c;s\x15\xdd\xa0\x10`\xac\xcb\xfa\x1bq\xfa\x0c\xa1\x81\xe6F@\x98DE\xa5I\x81\x1a\xf8\x97ai\x98?\x89\xd3\xa8\xb7\x1cg\xe8\xda\xdcpJ\xa4\x93#%\xf6\x81\xd0\x9a\x8bk ,s\xbe\xde\xc4\x9biI\xf9\xa6:\xe1!\xb0\x02Gb\x85;\xdc\xc3\0\xd6\n\xe49\xe8\xae\xb1\x076BM\x01~\x8cV\x7f`\xd3\x91\xa5\xfa\xbe?\x10\xaa\x147\x99\xb9\rC\xa1\xba+\xa2\xe1S5\x9dtW\n-\xda\x82\xb6\x88RG\x99\x873f\xcb\xdd\xaf>\xe1\x13\xed\xd4\xef\xbbJ\xb5H\"\x93;\xba\xbcy\xe8,jX\xa8D(cn\xa0\xba\xd9\xed\x98\xc3\xdf0O\xf94J\xde\x91\xccd\"J\x12\xc0\xe7\xac\x0bTtrc\x1fz\x0e\xb5\xbc\xb7\x81\x9a\xd5\x87\xeb\x1c\xb4\xc1!\xd8\xa7]\x8au\xc2\xac\xc6\x9dH\xf3\"\x18>\x13e\xdb\xa7\xa7\x1f\x94V\xf9\x17p\x8arhb\xb5tP\x0c\xc4\xda\x8d\xb5\x93:\x11\xf09T\x1d\x84\xc2@\xa9\x9a\x0e\xe44\t\x1b\xd7{\x12}^w\x07\xa1\x82\x84\nl~Q\xa4Z\x93\xa8\xdcc\x91Fz:\xb7}\x03\xff\x8b\x19Xl\xc1\xf1\x7fOR\xed\x9fU\r\x13'\xdf\xcdCoY\xfbS\x06\x0c\xdbXZ\xf5\xcb8\xcc[\x10\x9a\x10\xc9Dl\"\t\x10\x1a\x16k\x13\xc9*\x11;Rx\xc9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0bg\xc3\xd4\x90\xaf\xcd~5\x97#:F\xbb\xee\xc8\x8b\xac\xbc\xea\xe7\xf2a\xa2\xb0fk\xdf_*\x0c\xf7\xb1\r\x99\xa1J|\xfd\xae~M\xc5\x19'\x11\xa7\x8aq\"\xdb\xeev\x05F\xd0\x97\x82\xe7\xbds\x99Z\xf8\xf3\t\xedX\xd9\x92\x07#5\xd3\x03zD\xec\xfd\xe2\xdbV\xed\xbcK\x90*{\x9bw\x01[\x99:7\xbb\x98LL\xdf\xfb\x1a\xaf\xe2\xda>\x8fT`\0\xa2{\xb1\x84dAN$\xc6\xdf\xa7z\t\x1dW8d\xf2\x18H\xdb\xdcC\xdb>\xca\xb4\x92VA\xe9\xfb\xa6d\xff\xaa\xce\xbc\xc2\x92\xe3w\xdc_C\xa2h\xbf\xd7\x17\xf01\x93\xbd\xfa\xb4\xa8\xe38\xee\xbb\xf6<\x9b\xc1l\xca\xe8\xedR\xa7\xed\x07\xdbx\x17\x95Bx9\xdb\xa6\x83}\x06\x97 r\x02Z\xb7:\x87\x18\x97\x11\xd9H\xb6\x9b\xdalf'\xd4\x8a\x99H\x9a:\x07\x0c`\xb9b4\x80+\xae\x8d\x17\xf3\x8bE\ny\x17\0\xa2\xa1\x9e[\xfc(hA[\x86\x06\xcb\x0e\xbf\xc4|\x9b.\x9f1\xeak\x9ed\xdf\xe0c\xb5\xd2\xd8\xb7Il\xf9\xbb([\xbd\xef\xa3G`\xcd\xf6\x13\x8c;8\xf4\xee(kk\xdf\x97b\x1e\x1b\x9c*%\xea>E\xdc\xfe\xc7\x1a\xc3\x87cv\xcb\xa8O\xcd\xe0\x89L{\xec\x94k)\xd4\xce\xb22b\xff\n\xa63\xbc\xaa&\xf7\x17e\xce\xd5\xfc\x13\x1d\xab\xb3\xb8\x96Xw\x94b\x8bR\x0c".into();
    let new_price = get_chainlink_price_from_transmit_tx(&data, btc_token).await?;

    let btc_expected_price = BigDecimal::from_str("59017.88")?;

    assert_eq!(new_price, btc_expected_price);

    println!("updating token price");
    update_token_price_for_(btc_token, new_price, &client).await?;

    let new_price = get_saved_token_price(btc_token.address.clone()).await?;

    assert_eq!(new_price, btc_expected_price);

    Ok(())
}
