#[derive(Clone, Copy, Debug)]
pub struct ChainlinkPriceFeed {
    pub token_symbol: &'static str,
    pub base_currency: &'static str,
    pub address: &'static str,
}

pub static ETHEREUM_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
    ChainlinkPriceFeed {
        token_symbol: "1INCH",
        base_currency: "ETH",
        address: "0x72AFAECF99C9d9C8215fF44C77B94B99C28741e8",
    },
    ChainlinkPriceFeed {
        token_symbol: "1INCH",
        base_currency: "USD",
        address: "0xc929ad75B72593967DE83E7F7Cda0493458261D9",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "ETH",
        address: "0x6Df09E975c830ECae5bd4eD9d90f3A95a4f88012",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "USD",
        address: "0x547a514d5e3769680Ce22B2361c10Ea13619e8a9",
    },
    ChainlinkPriceFeed {
        token_symbol: "AGEUR",
        base_currency: "EUR",
        address: "0xb4d5289C58CE36080b0748B47F859D8F50dFAACb",
    },
    ChainlinkPriceFeed {
        token_symbol: "ALCX",
        base_currency: "ETH",
        address: "0x194a9AaF2e0b67c35915cD01101585A33Fe25CAa",
    },
    ChainlinkPriceFeed {
        token_symbol: "AMPL",
        base_currency: "ETH",
        address: "0x492575FDD11a0fCf2C6C719867890a7648d526eB",
    },
    ChainlinkPriceFeed {
        token_symbol: "AMPL",
        base_currency: "USD",
        address: "0xe20CA8D7546932360e37E9D72c1a47334af57706",
    },
    ChainlinkPriceFeed {
        token_symbol: "APE",
        base_currency: "ETH",
        address: "0xc7de7f4d4C9c991fF62a07D18b3E31e349833A18",
    },
    ChainlinkPriceFeed {
        token_symbol: "APE",
        base_currency: "USD",
        address: "0xD10aBbC76679a20055E167BB80A24ac851b37056",
    },
    ChainlinkPriceFeed {
        token_symbol: "ARB",
        base_currency: "USD",
        address: "0x31697852a68433DbCc2Ff612c516d69E3D9bd08F",
    },
    ChainlinkPriceFeed {
        token_symbol: "AUD",
        base_currency: "USD",
        address: "0x77F9710E7d0A19669A13c055F62cd80d313dF022",
    },
    ChainlinkPriceFeed {
        token_symbol: "AVAX",
        base_currency: "USD",
        address: "0xFF3EEb22B5E3dE6e705b44749C2559d704923FD7",
    },
    ChainlinkPriceFeed {
        token_symbol: "Arbitrum Healthcheck Sequencer Flag",
        base_currency: "",
        address: "0x32EaFC72772821936BCc9b8A32dC394fEFcDBfD9",
    },
    ChainlinkPriceFeed {
        token_symbol: "BADGER",
        base_currency: "ETH",
        address: "0x58921Ac140522867bf50b9E009599Da0CA4A2379",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAL",
        base_currency: "ETH",
        address: "0xC1438AA3823A6Ba0C159CfA8D98dF5A994bA120b",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAL",
        base_currency: "USD",
        address: "0xdF2917806E30300537aEB49A7663062F4d1F2b5F",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAT",
        base_currency: "ETH",
        address: "0x0d16d4528239e9ee52fa531af613AcdB23D88c94",
    },
    ChainlinkPriceFeed {
        token_symbol: "BNB",
        base_currency: "USD",
        address: "0x14e613AC84a31f709eadbdF89C6CC390fDc9540A",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "ETH",
        address: "0xdeb288F737066589598e9214E782fa5A8eD689e8",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "USD",
        address: "0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c",
    },
    ChainlinkPriceFeed {
        token_symbol: "C3M",
        base_currency: "EUR",
        address: "0xD41390267Afec3fA5b4c0B3aA6c706556CCE75ec",
    },
    ChainlinkPriceFeed {
        token_symbol: "CAD",
        base_currency: "USD",
        address: "0xa34317DB73e77d453b1B8d04550c44D10e981C8e",
    },
    ChainlinkPriceFeed {
        token_symbol: "CBETH",
        base_currency: "ETH",
        address: "0xF017fcB346A1885194689bA23Eff2fE6fA5C483b",
    },
    ChainlinkPriceFeed {
        token_symbol: "CHF",
        base_currency: "USD",
        address: "0x449d117117838fFA61263B61dA6301AA2a88B13A",
    },
    ChainlinkPriceFeed {
        token_symbol: "CNY",
        base_currency: "USD",
        address: "0xeF8A4aF35cd47424672E3C590aBD37FBB7A7759a",
    },
    ChainlinkPriceFeed {
        token_symbol: "COMP",
        base_currency: "ETH",
        address: "0x1B39Ee86Ec5979ba5C322b826B3ECb8C79991699",
    },
    ChainlinkPriceFeed {
        token_symbol: "COMP",
        base_currency: "USD",
        address: "0xdbd020CAeF83eFd542f4De03e3cF0C28A4428bd5",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "ETH",
        address: "0x8a12Be339B0cD1829b91Adc01977caa5E9ac121e",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "USD",
        address: "0xCd627aA160A6fA45Eb793D19Ef54f5062F20f33f",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRVUSD",
        base_currency: "USD",
        address: "0xEEf0C605546958c1f899b6fB336C20671f9cD49F",
    },
    ChainlinkPriceFeed {
        token_symbol: "CSPX",
        base_currency: "USD",
        address: "0xF4E1B57FB228879D057ac5AE33973e8C53e4A0e0",
    },
    ChainlinkPriceFeed {
        token_symbol: "CVX",
        base_currency: "ETH",
        address: "0xC9CbF687f43176B302F03f5e58470b77D07c61c6",
    },
    ChainlinkPriceFeed {
        token_symbol: "CVX",
        base_currency: "USD",
        address: "0xd962fC30A72A84cE50161031391756Bf2876Af5D",
    },
    ChainlinkPriceFeed {
        token_symbol: "Calculated XSUSHI",
        base_currency: "ETH",
        address: "0xF05D9B6C08757EAcb1fbec18e36A1B7566a13DEB",
    },
    ChainlinkPriceFeed {
        token_symbol: "Consumer Price Index",
        base_currency: "",
        address: "0x9a51192e065ECC6BDEafE5e194ce54702DE4f1f5",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "ETH",
        address: "0x773616E4d11A78F511299002da57A0a94577F1f4",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "USD",
        address: "0xAed0c38402a5d19df6E4c03F4E2DceD6e29c1ee9",
    },
    ChainlinkPriceFeed {
        token_symbol: "DPI",
        base_currency: "ETH",
        address: "0x029849bbc0b1d93b85a8b6190e979fd38F5760E2",
    },
    ChainlinkPriceFeed {
        token_symbol: "DPI",
        base_currency: "USD",
        address: "0xD2A593BF7594aCE1faD597adb697b5645d5edDB2",
    },
    ChainlinkPriceFeed {
        token_symbol: "ENJ",
        base_currency: "ETH",
        address: "0x24D9aB51950F3d62E9144fdC2f3135DAA6Ce8D1B",
    },
    ChainlinkPriceFeed {
        token_symbol: "ENS",
        base_currency: "USD",
        address: "0x5C00128d4d1c2F4f652C267d7bcdD7aC99C16E16",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "BTC",
        address: "0xAc559F25B1619171CbC396a50854A3240b6A4e99",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "USD",
        address: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETHx",
        base_currency: "ETH",
        address: "0xC5f8c4aB091Be1A899214c0C3636ca33DcA0C547",
    },
    ChainlinkPriceFeed {
        token_symbol: "EUR",
        base_currency: "USD",
        address: "0xb49f677943BC038e9857d61E7d053CaA2C1734C1",
    },
    ChainlinkPriceFeed {
        token_symbol: "FDUSD",
        base_currency: "USD",
        address: "0xfAA9147190c2C2cc5B8387B4f49016bDB3380572",
    },
    ChainlinkPriceFeed {
        token_symbol: "FIL",
        base_currency: "ETH",
        address: "0x0606Be69451B1C9861Ac6b3626b99093b713E801",
    },
    ChainlinkPriceFeed {
        token_symbol: "FRAX",
        base_currency: "ETH",
        address: "0x14d04Fff8D21bd62987a5cE9ce543d2F1edF5D3E",
    },
    ChainlinkPriceFeed {
        token_symbol: "FRAX",
        base_currency: "USD",
        address: "0xB9E1E3A9feFf48998E45Fa90847ed4D467E8BcfD",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTM",
        base_currency: "ETH",
        address: "0x2DE7E4a9488488e0058B95854CC2f7955B35dC9b",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTT",
        base_currency: "ETH",
        address: "0xF0985f7E2CaBFf22CecC5a71282a89582c382EFE",
    },
    ChainlinkPriceFeed {
        token_symbol: "FXS",
        base_currency: "USD",
        address: "0x6Ebc52C8C1089be9eB3945C4350B68B8E4C2233f",
    },
    ChainlinkPriceFeed {
        token_symbol: "GBP",
        base_currency: "USD",
        address: "0x5c0Ab2d9b5a7ed9f470386e82BB36A3613cDd4b5",
    },
    ChainlinkPriceFeed {
        token_symbol: "GHO",
        base_currency: "USD",
        address: "0x3f12643D3f6f874d39C2a4c9f2Cd6f2DbAC877FC",
    },
    ChainlinkPriceFeed {
        token_symbol: "GRT",
        base_currency: "ETH",
        address: "0x17D054eCac33D91F7340645341eFB5DE9009F1C1",
    },
    ChainlinkPriceFeed {
        token_symbol: "GRT",
        base_currency: "USD",
        address: "0x86cF33a451dE9dc61a2862FD94FF4ad4Bd65A5d2",
    },
    ChainlinkPriceFeed {
        token_symbol: "HIGH",
        base_currency: "USD",
        address: "0x5C8D8AaB4ffa4652753Df94f299330Bb4479bF85",
    },
    ChainlinkPriceFeed {
        token_symbol: "IB01",
        base_currency: "USD",
        address: "0x32d1463EB53b73C095625719Afa544D5426354cB",
    },
    ChainlinkPriceFeed {
        token_symbol: "IBTA",
        base_currency: "USD",
        address: "0xd27e6D02b72eB6FCe04Ad5690C419196B4EF2885",
    },
    ChainlinkPriceFeed {
        token_symbol: "IMX",
        base_currency: "USD",
        address: "0xBAEbEFc1D023c0feCcc047Bff42E75F15Ff213E6",
    },
    ChainlinkPriceFeed {
        token_symbol: "JPY",
        base_currency: "USD",
        address: "0xBcE206caE7f0ec07b545EddE332A47C2F75bbeb3",
    },
    ChainlinkPriceFeed {
        token_symbol: "KNC",
        base_currency: "ETH",
        address: "0x656c0544eF4C98A6a98491833A89204Abb045d6b",
    },
    ChainlinkPriceFeed {
        token_symbol: "KNC",
        base_currency: "USD",
        address: "0xf8fF43E991A81e6eC886a3D281A2C6cC19aE70Fc",
    },
    ChainlinkPriceFeed {
        token_symbol: "KRW",
        base_currency: "USD",
        address: "0x01435677FB11763550905594A16B645847C1d0F3",
    },
    ChainlinkPriceFeed {
        token_symbol: "LDO",
        base_currency: "ETH",
        address: "0x4e844125952D32AcdF339BE976c98E22F6F318dB",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "ETH",
        address: "0xDC530D9457755926550b59e8ECcdaE7624181557",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "USD",
        address: "0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c",
    },
    ChainlinkPriceFeed {
        token_symbol: "LUSD",
        base_currency: "USD",
        address: "0x3D7aE7E594f2f2091Ad8798313450130d0Aba3a0",
    },
    ChainlinkPriceFeed {
        token_symbol: "MANA",
        base_currency: "ETH",
        address: "0x82A44D92D6c329826dc557c5E1Be6ebeC5D5FeB9",
    },
    ChainlinkPriceFeed {
        token_symbol: "MATIC",
        base_currency: "USD",
        address: "0x7bAC85A8a13A4BcD8abb3eB7d6b4d632c5a57676",
    },
    ChainlinkPriceFeed {
        token_symbol: "MAVIA",
        base_currency: "USD",
        address: "0x29d26C008e8f201eD0D864b1Fd9392D29d0C8e96",
    },
    ChainlinkPriceFeed {
        token_symbol: "MKR",
        base_currency: "ETH",
        address: "0x24551a8Fb2A7211A25a17B1481f043A8a8adC7f2",
    },
    ChainlinkPriceFeed {
        token_symbol: "MKR",
        base_currency: "USD",
        address: "0xec1D1B3b0443256cc3860e24a46F108e699484Aa",
    },
    ChainlinkPriceFeed {
        token_symbol: "MLN",
        base_currency: "ETH",
        address: "0xDaeA8386611A157B08829ED4997A8A62B557014C",
    },
    ChainlinkPriceFeed {
        token_symbol: "NZD",
        base_currency: "USD",
        address: "0x3977CFc9e4f29C184D4675f4EB8e0013236e5f3e",
    },
    ChainlinkPriceFeed {
        token_symbol: "Nexus wETH Reserves",
        base_currency: "",
        address: "0xCc72039A141c6e34a779eF93AEF5eB4C82A893c7",
    },
    ChainlinkPriceFeed {
        token_symbol: "PAX",
        base_currency: "ETH",
        address: "0x3a08ebBaB125224b7b6474384Ee39fBb247D2200",
    },
    ChainlinkPriceFeed {
        token_symbol: "PAXG",
        base_currency: "ETH",
        address: "0x9B97304EA12EFed0FAd976FBeCAad46016bf269e",
    },
    ChainlinkPriceFeed {
        token_symbol: "PERP",
        base_currency: "ETH",
        address: "0x3b41D5571468904D4e53b6a8d93A6BaC43f02dC9",
    },
    ChainlinkPriceFeed {
        token_symbol: "PYUSD",
        base_currency: "USD",
        address: "0x8f1dF6D7F2db73eECE86a18b4381F4707b918FB1",
    },
    ChainlinkPriceFeed {
        token_symbol: "RDNT",
        base_currency: "USD",
        address: "0x393CC05baD439c9B36489384F11487d9C8410471",
    },
    ChainlinkPriceFeed {
        token_symbol: "REN",
        base_currency: "ETH",
        address: "0x3147D7203354Dc06D9fd350c7a2437bcA92387a4",
    },
    ChainlinkPriceFeed {
        token_symbol: "RETH",
        base_currency: "ETH",
        address: "0x536218f9E9Eb48863970252233c8F271f554C2d0",
    },
    ChainlinkPriceFeed {
        token_symbol: "RPL",
        base_currency: "USD",
        address: "0x4E155eD98aFE9034b7A5962f6C84c86d869daA9d",
    },
    ChainlinkPriceFeed {
        token_symbol: "RSETH",
        base_currency: "ETH",
        address: "0x03c68933f7a3F76875C0bc670a58e69294cDFD01",
    },
    ChainlinkPriceFeed {
        token_symbol: "RSR",
        base_currency: "USD",
        address: "0x759bBC1be8F90eE6457C44abc7d443842a976d02",
    },
    ChainlinkPriceFeed {
        token_symbol: "SAND",
        base_currency: "USD",
        address: "0x35E3f7E558C04cE7eEE1629258EcbbA03B36Ec56",
    },
    ChainlinkPriceFeed {
        token_symbol: "SGD",
        base_currency: "USD",
        address: "0xe25277fF4bbF9081C75Ab0EB13B4A13a721f3E13",
    },
    ChainlinkPriceFeed {
        token_symbol: "SHIB",
        base_currency: "ETH",
        address: "0x8dD1CD88F43aF196ae478e91b9F5E4Ac69A97C61",
    },
    ChainlinkPriceFeed {
        token_symbol: "SHV",
        base_currency: "USD",
        address: "0xc04611C43842220fd941515F86d1DDdB15F04e46",
    },
    ChainlinkPriceFeed {
        token_symbol: "SNX",
        base_currency: "ETH",
        address: "0x79291A9d692Df95334B1a0B3B4AE6bC606782f8c",
    },
    ChainlinkPriceFeed {
        token_symbol: "SNX",
        base_currency: "USD",
        address: "0xDC3EA94CD0AC27d9A86C180091e7f78C683d3699",
    },
    ChainlinkPriceFeed {
        token_symbol: "SOL",
        base_currency: "USD",
        address: "0x4ffC43a60e009B551865A93d232E33Fce9f01507",
    },
    ChainlinkPriceFeed {
        token_symbol: "SPELL",
        base_currency: "USD",
        address: "0x8c110B94C5f1d347fAcF5E1E938AB2db60E3c9a8",
    },
    ChainlinkPriceFeed {
        token_symbol: "STETH",
        base_currency: "ETH",
        address: "0x86392dC19c0b719886221c78AB11eb8Cf5c52812",
    },
    ChainlinkPriceFeed {
        token_symbol: "STETH",
        base_currency: "USD",
        address: "0xCfE54B5cD566aB89272946F602D76Ea879CAb4a8",
    },
    ChainlinkPriceFeed {
        token_symbol: "STG",
        base_currency: "USD",
        address: "0x7A9f34a0Aa917D438e9b6E630067062B7F8f6f3d",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSD",
        base_currency: "ETH",
        address: "0x8e0b7e6062272B5eF4524250bFFF8e5Bd3497757",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "ETH",
        address: "0xe572CeF69f43c2E488b33924AF04BDacE19079cf",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "USD",
        address: "0xCc70F09A6CC17553b2E31954cD36E4A2d89501f7",
    },
    ChainlinkPriceFeed {
        token_symbol: "SWETH",
        base_currency: "ETH",
        address: "0xec21B3e882CE09928cb397DcfF31B15cBBD1e1C3",
    },
    ChainlinkPriceFeed {
        token_symbol: "SXP",
        base_currency: "USD",
        address: "0xFb0CfD6c19e25DB4a08D8a204a387cEa48Cc138f",
    },
    ChainlinkPriceFeed {
        token_symbol: "TAO",
        base_currency: "USD",
        address: "0x1c88503c9A52aE6aaE1f9bb99b3b7e9b8Ab35459",
    },
    ChainlinkPriceFeed {
        token_symbol: "TBTC",
        base_currency: "USD",
        address: "0x8350b7De6a6a2C1368E7D4Bd968190e13E354297",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRY",
        base_currency: "USD",
        address: "0xB09fC5fD3f11Cf9eb5E1C5Dba43114e3C9f477b5",
    },
    ChainlinkPriceFeed {
        token_symbol: "TUSD",
        base_currency: "ETH",
        address: "0x3886BA987236181D98F2401c507Fb8BeA7871dF2",
    },
    ChainlinkPriceFeed {
        token_symbol: "TUSD",
        base_currency: "USD",
        address: "0xec746eCF986E2927Abd291a2A1716c940100f8Ba",
    },
    ChainlinkPriceFeed {
        token_symbol: "Total Marketcap",
        base_currency: "USD",
        address: "0xEC8761a0A73c34329CA5B1D3Dc7eD07F30e836e2",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "ETH",
        address: "0xD6aA3D25116d8dA79Ea0246c4826EB951872e02e",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "USD",
        address: "0x553303d460EE0afB37EdFf9bE42922D8FF63220e",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "ETH",
        address: "0x986b5E1e1755e3C2440e960477f25201B0a8bbD4",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "USD",
        address: "0x8fFfFfd4AfB6115b954Bd326cbe7B4BA576818f6",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDD",
        base_currency: "USD",
        address: "0x0ed39A19D2a68b722408d84e4d970827f61E6c0A",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDP",
        base_currency: "USD",
        address: "0x09023c0DA49Aaf8fc3fA3ADF34C6A7016D38D5e3",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "ETH",
        address: "0xEe9F2375b4bdF6387aa8265dD4FB8F16512A1d46",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "USD",
        address: "0x3E7d1eAB13ad0104d2750B8863b489D65364e32D",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDV",
        base_currency: "USD",
        address: "0x925B831EB4c9fFA7e384254fb2cd508c65FAe3FE",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDe",
        base_currency: "USD",
        address: "0xa569d910839Ae8865Da8F8e70FfFb0cBA869F961",
    },
    ChainlinkPriceFeed {
        token_symbol: "WBTC",
        base_currency: "BTC",
        address: "0xfdFD9C85aD200c506Cf9e21F1FD8dd01932FBB23",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAG",
        base_currency: "USD",
        address: "0x379589227b15F1a12195D3f2d90bBc9F31f95235",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAU",
        base_currency: "USD",
        address: "0x214eD9Da11D2fbe465a6fc601a91E62EbEc1a0D6",
    },
    ChainlinkPriceFeed {
        token_symbol: "XCN",
        base_currency: "USD",
        address: "0xeb988B77b94C186053282BfcD8B7ED55142D3cAB",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "ETH",
        address: "0x7c5d4F8345e66f68099581Db340cd65B078C41f4",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "USD",
        address: "0xA027702dbb89fbd58938e4324ac03B58d812b0E1",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZRX",
        base_currency: "ETH",
        address: "0x2Da4983a622a8498bb1a21FaE9D8F6C664939962",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZRX",
        base_currency: "USD",
        address: "0x2885d15b8Af22648b98B122b22FDF4D2a56c6023",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0x636A000262F6aA9e1F094ABF0aD8f645C44f641C",
    },
    ChainlinkPriceFeed {
        token_symbol: "mETH",
        base_currency: "ETH",
        address: "0x5b563107C8666d2142C216114228443B94152362",
    },
    ChainlinkPriceFeed {
        token_symbol: "rswETH",
        base_currency: "ETH",
        address: "0xb613CfebD0b6e95abDDe02677d6bC42394FdB857",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USD",
        address: "0xFF3BC18cCBd5999CE63E788A1c250a88626aD099",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "ETH",
        address: "0x5c9C449BbC9a6075A2c061dF312a35fd1E05fF22",
    },
];

pub static POLYGON_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
    ChainlinkPriceFeed {
        token_symbol: "1INCH",
        base_currency: "USD",
        address: "0x443C5116CdF663Eb387e72C688D276e702135C87",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAPL",
        base_currency: "USD",
        address: "0x7E7B45b08F68EC69A99AAb12e42FcCB078e10094",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "ETH",
        address: "0xbE23a3AA13038CfC28aFd0ECe4FdE379fE7fBfc4",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "USD",
        address: "0x72484B12719E23115761D5DA1646945632979bB6",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE Network Emergency Count (Polygon)",
        base_currency: "",
        address: "0xDAFA1989A504c48Ee20a582f2891eeB25E2fA23F",
    },
    ChainlinkPriceFeed {
        token_symbol: "ADA",
        base_currency: "USD",
        address: "0x882554df528115a743c4537828DA8D5B58e52544",
    },
    ChainlinkPriceFeed {
        token_symbol: "AGEUR",
        base_currency: "USD",
        address: "0x9b88d07B2354eF5f4579690356818e07371c7BeD",
    },
    ChainlinkPriceFeed {
        token_symbol: "ALCX",
        base_currency: "USD",
        address: "0x5DB6e61B6159B20F068dc15A47dF2E5931b14f29",
    },
    ChainlinkPriceFeed {
        token_symbol: "ALGO",
        base_currency: "USD",
        address: "0x03Bc6D9EFed65708D35fDaEfb25E87631a0a3437",
    },
    ChainlinkPriceFeed {
        token_symbol: "AMZN",
        base_currency: "USD",
        address: "0xf9184b8E5da48C19fA4E06f83f77742e748cca96",
    },
    ChainlinkPriceFeed {
        token_symbol: "APE",
        base_currency: "USD",
        address: "0x2Ac3F3Bfac8fC9094BC3f0F9041a51375235B992",
    },
    ChainlinkPriceFeed {
        token_symbol: "AUD",
        base_currency: "USD",
        address: "0x062Df9C4efd2030e243ffCc398b652e8b8F95C6f",
    },
    ChainlinkPriceFeed {
        token_symbol: "AVAX",
        base_currency: "USD",
        address: "0xe01eA2fbd8D76ee323FbEd03eB9a8625EC981A10",
    },
    ChainlinkPriceFeed {
        token_symbol: "AXS",
        base_currency: "USD",
        address: "0x9c371aE34509590E10aB98205d2dF5936A1aD875",
    },
    ChainlinkPriceFeed {
        token_symbol: "BADGER",
        base_currency: "ETH",
        address: "0x82C9d4E88862f194C2bd874a106a90dDD0D35AAB",
    },
    ChainlinkPriceFeed {
        token_symbol: "BADGER",
        base_currency: "USD",
        address: "0xF626964Ba5e81405f47e8004F0b276Bb974742B5",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAL",
        base_currency: "ETH",
        address: "0x03CD157746c61F44597dD54C6f6702105258C722",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAL",
        base_currency: "USD",
        address: "0xD106B538F2A868c28Ca1Ec7E298C3325E0251d66",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAT",
        base_currency: "USD",
        address: "0x2346Ce62bd732c62618944E51cbFa09D985d86D2",
    },
    ChainlinkPriceFeed {
        token_symbol: "BCH",
        base_currency: "USD",
        address: "0x327d9822e9932996f55b39F557AEC838313da8b7",
    },
    ChainlinkPriceFeed {
        token_symbol: "BNB",
        base_currency: "USD",
        address: "0x82a6c4AF830caa6c97bb504425f6A66165C2c26e",
    },
    ChainlinkPriceFeed {
        token_symbol: "BRL",
        base_currency: "USD",
        address: "0xB90DA3ff54C3ED09115abf6FbA0Ff4645586af2c",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "ETH",
        address: "0x19b0F0833C78c0848109E3842D34d2fDF2cA69BA",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "USD",
        address: "0xc907E116054Ad103354f2D350FD2514433D57F6f",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC-USD Total Marketcap",
        base_currency: "USD",
        address: "0x18E4058491C3F58bC2f747A9E64cA256Ed6B318d",
    },
    ChainlinkPriceFeed {
        token_symbol: "CAD",
        base_currency: "USD",
        address: "0xACA44ABb8B04D07D883202F99FA5E3c53ed57Fb5",
    },
    ChainlinkPriceFeed {
        token_symbol: "CBETH",
        base_currency: "ETH",
        address: "0x0a6a03CdF7d0b48d4e4BA8e362A4FfC3aAC4f3c0",
    },
    ChainlinkPriceFeed {
        token_symbol: "CHF",
        base_currency: "USD",
        address: "0xc76f762CedF0F78a439727861628E0fdfE1e70c2",
    },
    ChainlinkPriceFeed {
        token_symbol: "CHZ",
        base_currency: "USD",
        address: "0x2409987e514Ad8B0973C2b90ee1D95051DF0ECB9",
    },
    ChainlinkPriceFeed {
        token_symbol: "CNY",
        base_currency: "USD",
        address: "0x04bB437Aa63E098236FA47365f0268547f6EAB32",
    },
    ChainlinkPriceFeed {
        token_symbol: "COMP",
        base_currency: "USD",
        address: "0x2A8758b7257102461BC958279054e372C2b1bDE6",
    },
    ChainlinkPriceFeed {
        token_symbol: "COP",
        base_currency: "USD",
        address: "0xfAA9147190c2C2cc5B8387B4f49016bDB3380572",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "ETH",
        address: "0x1CF68C76803c9A415bE301f50E82e44c64B7F1D4",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "USD",
        address: "0x336584C8E6Dc19637A5b36206B1c79923111b405",
    },
    ChainlinkPriceFeed {
        token_symbol: "CVX",
        base_currency: "USD",
        address: "0x5ec151834040B4D453A1eA46aA634C1773b36084",
    },
    ChainlinkPriceFeed {
        token_symbol: "Calculated MaticX",
        base_currency: "USD",
        address: "0x5d37E4b374E6907de8Fc7fb33EE3b0af403C7403",
    },
    ChainlinkPriceFeed {
        token_symbol: "Calculated stMATIC",
        base_currency: "USD",
        address: "0x97371dF4492605486e23Da797fA68e55Fc38a13f",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "ETH",
        address: "0xFC539A559e170f848323e19dfD66007520510085",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "USD",
        address: "0x4746DeC9e833A82EC7C2C1356372CcF2cfcD2F3D",
    },
    ChainlinkPriceFeed {
        token_symbol: "DASH",
        base_currency: "USD",
        address: "0xD94427eDee70E4991b4b8DdCc848f2B58ED01C0b",
    },
    ChainlinkPriceFeed {
        token_symbol: "DGB",
        base_currency: "USD",
        address: "0x4205eC5fd179A843caa7B0860a8eC7D980013359",
    },
    ChainlinkPriceFeed {
        token_symbol: "DODO",
        base_currency: "USD",
        address: "0x59161117086a4C7A9beDA16C66e40Bdaa1C5a8B6",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOGE",
        base_currency: "USD",
        address: "0xbaf9327b6564454F4a3364C33eFeEf032b4b4444",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOGE-USD Total Marketcap",
        base_currency: "USD",
        address: "0xbd238a35Fb47aE22F0cC551f14ffB8E8f04FCA21",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOT",
        base_currency: "USD",
        address: "0xacb51F1a83922632ca02B25a8164c10748001BdE",
    },
    ChainlinkPriceFeed {
        token_symbol: "DPI",
        base_currency: "ETH",
        address: "0xC70aAF9092De3a4E5000956E672cDf5E996B4610",
    },
    ChainlinkPriceFeed {
        token_symbol: "DPI",
        base_currency: "USD",
        address: "0x2e48b7924FBe04d575BA229A59b64547d9da16e9",
    },
    ChainlinkPriceFeed {
        token_symbol: "EOS",
        base_currency: "USD",
        address: "0xd6285F06203D938ab713Fa6A315e7d23247DDE95",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETC",
        base_currency: "USD",
        address: "0xDf3f72Be10d194b58B1BB56f2c4183e661cB2114",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "USD",
        address: "0xF9680D99D6C9589e2a93a78A04A279e509205945",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH-USD Total Marketcap",
        base_currency: "USD",
        address: "0x67935f65D1577ced9f4929D3679A157E95C1c02c",
    },
    ChainlinkPriceFeed {
        token_symbol: "EUR",
        base_currency: "USD",
        address: "0x73366Fe0AA0Ded304479862808e02506FE556a98",
    },
    ChainlinkPriceFeed {
        token_symbol: "FB",
        base_currency: "USD",
        address: "0x5b4586C911144A947D7814Fd71fe0872b8334748",
    },
    ChainlinkPriceFeed {
        token_symbol: "FIL",
        base_currency: "USD",
        address: "0xa07703E5C2eD1516107c7c72A494493Dcb99C676",
    },
    ChainlinkPriceFeed {
        token_symbol: "FRAX",
        base_currency: "USD",
        address: "0x00DBeB1e45485d53DF7C2F0dF1Aa0b6Dc30311d3",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTM",
        base_currency: "USD",
        address: "0x58326c0F831b2Dbf7234A4204F28Bba79AA06d5f",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTT / ",
        base_currency: "USD",
        address: "0x817A7D43f0277Ca480AE03Ec76Fc63A2EC7114bA",
    },
    ChainlinkPriceFeed {
        token_symbol: "FXS",
        base_currency: "USD",
        address: "0x6C0fe985D3cAcbCdE428b84fc9431792694d0f51",
    },
    ChainlinkPriceFeed {
        token_symbol: "GBP",
        base_currency: "USD",
        address: "0x099a2540848573e94fb1Ca0Fa420b00acbBc845a",
    },
    ChainlinkPriceFeed {
        token_symbol: "GHST",
        base_currency: "ETH",
        address: "0xe638249AF9642CdA55A92245525268482eE4C67b",
    },
    ChainlinkPriceFeed {
        token_symbol: "GHST",
        base_currency: "USD",
        address: "0xDD229Ce42f11D8Ee7fFf29bDB71C7b81352e11be",
    },
    ChainlinkPriceFeed {
        token_symbol: "GNS",
        base_currency: "USD",
        address: "0x9cb43aa3D036Cb035a694Ba0AAa91f8875B16cE1",
    },
    ChainlinkPriceFeed {
        token_symbol: "GOOGL",
        base_currency: "USD",
        address: "0x1b32682C033b2DD7EFdC615FA82d353e254F39b5",
    },
    ChainlinkPriceFeed {
        token_symbol: "GRT",
        base_currency: "USD",
        address: "0x3FabBfb300B1e2D7c9B84512fe9D30aeDF24C410",
    },
    ChainlinkPriceFeed {
        token_symbol: "HBAR",
        base_currency: "USD",
        address: "0xC5878bDf8a89FA3bF0DC8389ae8EE6DE601D01bC",
    },
    ChainlinkPriceFeed {
        token_symbol: "ICP",
        base_currency: "USD",
        address: "0x84227A76a04289473057BEF706646199D7C58c34",
    },
    ChainlinkPriceFeed {
        token_symbol: "ILS",
        base_currency: "USD",
        address: "0x8d5eB34C509261533235b91350d359EdcB969D33",
    },
    ChainlinkPriceFeed {
        token_symbol: "ILV",
        base_currency: "ETH",
        address: "0x3636B780588328dc3F5df075De5627DBc9A6BA10",
    },
    ChainlinkPriceFeed {
        token_symbol: "INR",
        base_currency: "USD",
        address: "0xDA0F8Df6F5dB15b346f4B8D1156722027E194E60",
    },
    ChainlinkPriceFeed {
        token_symbol: "JPY",
        base_currency: "USD",
        address: "0xD647a6fC9BC6402301583C91decC5989d8Bc382D",
    },
    ChainlinkPriceFeed {
        token_symbol: "KAVA",
        base_currency: "USD",
        address: "0x7899dd75C329eFe63e35b02bC7d60D3739FB23c5",
    },
    ChainlinkPriceFeed {
        token_symbol: "KLAY",
        base_currency: "USD",
        address: "0x86F87CB74238a6f24606534A2fCc05469Eb2bcF5",
    },
    ChainlinkPriceFeed {
        token_symbol: "KNC",
        base_currency: "USD",
        address: "0x10e5f3DFc81B3e5Ef4e648C4454D04e79E1E41E2",
    },
    ChainlinkPriceFeed {
        token_symbol: "KRW",
        base_currency: "USD",
        address: "0x24B820870F726dA9B0D83B0B28a93885061dbF50",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "ETH",
        address: "0xb77fa460604b9C6435A235D057F7D319AC83cb53",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "MATIC",
        address: "0x5787BefDc0ECd210Dfa948264631CD53E68F7802",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "USD",
        address: "0xd9FFdb71EbE7496cC440152d43986Aae0AB76665",
    },
    ChainlinkPriceFeed {
        token_symbol: "LTC",
        base_currency: "USD",
        address: "0xEB99F173cf7d9a6dC4D889C2Ad7103e8383b6Efa",
    },
    ChainlinkPriceFeed {
        token_symbol: "MANA",
        base_currency: "USD",
        address: "0xA1CbF3Fe43BC3501e3Fc4b573e822c70e76A7512",
    },
    ChainlinkPriceFeed {
        token_symbol: "MATIC",
        base_currency: "ETH",
        address: "0x327e23A4855b6F663a28c5161541d69Af8973302",
    },
    ChainlinkPriceFeed {
        token_symbol: "MATIC",
        base_currency: "USD",
        address: "0xAB594600376Ec9fD91F8e885dADF0CE036862dE0",
    },
    ChainlinkPriceFeed {
        token_symbol: "MKR",
        base_currency: "ETH",
        address: "0x807b59d12520830D1864286FA0271c27baa94197",
    },
    ChainlinkPriceFeed {
        token_symbol: "MKR",
        base_currency: "USD",
        address: "0xa070427bF5bA5709f70e98b94Cb2F435a242C46C",
    },
    ChainlinkPriceFeed {
        token_symbol: "MLN",
        base_currency: "ETH",
        address: "0xB89D583B72aBF9C3a7e6e093251C2fCad3365312",
    },
    ChainlinkPriceFeed {
        token_symbol: "MSFT / ",
        base_currency: "USD",
        address: "0xC43081d9EA6d1c53f1F0e525504d47Dd60de12da",
    },
    ChainlinkPriceFeed {
        token_symbol: "MXN / ",
        base_currency: "USD",
        address: "0x171b16562EA3476F5C61d1b8dad031DbA0768545",
    },
    ChainlinkPriceFeed {
        token_symbol: "NZD",
        base_currency: "USD",
        address: "0xa302a0B8a499fD0f00449df0a490DedE21105955",
    },
    ChainlinkPriceFeed {
        token_symbol: "OGN",
        base_currency: "USD",
        address: "0x8Ec0eC2e0F26D8253ABf39Db4B1793D76B49C6D5",
    },
    ChainlinkPriceFeed {
        token_symbol: "OHM Index",
        base_currency: "",
        address: "0xc08f70c26ab8C659EaF259c51a0F7ae22758c6ac",
    },
    ChainlinkPriceFeed {
        token_symbol: "OHMv2",
        base_currency: "USD",
        address: "0x4cE90F28C6357A7d3F47D680723d18AF3684cD00",
    },
    ChainlinkPriceFeed {
        token_symbol: "OM",
        base_currency: "USD",
        address: "0xc86105DccF9BD629Cea7Fd41f94c6050bF96D57F",
    },
    ChainlinkPriceFeed {
        token_symbol: "PAXG / ",
        base_currency: "USD",
        address: "0x0f6914d8e7e1214CDb3A4C6fbf729b75C69DF608",
    },
    ChainlinkPriceFeed {
        token_symbol: "PHP",
        base_currency: "USD",
        address: "0x218231089Bebb2A31970c3b77E96eCfb3BA006D1",
    },
    ChainlinkPriceFeed {
        token_symbol: "PLN",
        base_currency: "USD",
        address: "0xB34BCE11040702f71c11529D00179B2959BcE6C0",
    },
    ChainlinkPriceFeed {
        token_symbol: "QNT",
        base_currency: "USD",
        address: "0xF7F291042F6Cbc4deC0Ad75c17786511a530dbe8",
    },
    ChainlinkPriceFeed {
        token_symbol: "QUICK",
        base_currency: "USD",
        address: "0x2251169D32E7538652a9a8c86bf0c43bFcd956f1",
    },
    ChainlinkPriceFeed {
        token_symbol: "SAND",
        base_currency: "USD",
        address: "0x3D49406EDd4D52Fb7FFd25485f32E073b529C924",
    },
    ChainlinkPriceFeed {
        token_symbol: "SEK",
        base_currency: "USD",
        address: "0xbd92B4919ae82be8473859295dEF0e778A626302",
    },
    ChainlinkPriceFeed {
        token_symbol: "SGD",
        base_currency: "USD",
        address: "0x8CE3cAc0E6635ce04783709ca3CC4F5fc5304299",
    },
    ChainlinkPriceFeed {
        token_symbol: "SHIB",
        base_currency: "USD",
        address: "0x3710abeb1A0Fc7C2EC59C26c8DAA7a448ff6125A",
    },
    ChainlinkPriceFeed {
        token_symbol: "SLP",
        base_currency: "USD",
        address: "0xBB3eF70953fC3766bec4Ab7A9BF05B6E4caf89c6",
    },
    ChainlinkPriceFeed {
        token_symbol: "SNX",
        base_currency: "USD",
        address: "0xbF90A5D9B6EE9019028dbFc2a9E50056d5252894",
    },
    ChainlinkPriceFeed {
        token_symbol: "SOL / ",
        base_currency: "USD",
        address: "0x10C8264C0935b3B9870013e057f330Ff3e9C56dC",
    },
    ChainlinkPriceFeed {
        token_symbol: "SPY.US",
        base_currency: "",
        address: "0x187c42f6C0e7395AeA00B1B30CB0fF807ef86d5d",
    },
    ChainlinkPriceFeed {
        token_symbol: "STORJ",
        base_currency: "USD",
        address: "0x0F1d5Bd7be9B30Fc09E110cd6504Bd450e53cb0E",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "ETH",
        address: "0x17414Eb5159A082e8d41D243C1601c2944401431",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "USD",
        address: "0x49B0c695039243BBfEb8EcD054EB70061fd54aa0",
    },
    ChainlinkPriceFeed {
        token_symbol: "THB",
        base_currency: "USD",
        address: "0x5164Ad28fb12a5e55946090Ec3eE1B748AFb3785",
    },
    ChainlinkPriceFeed {
        token_symbol: "THETA",
        base_currency: "USD",
        address: "0x38611b09F8f2D520c14eA973765C225Bf57B9Eac",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRUMATIC",
        base_currency: "MATIC Exchange Rate",
        address: "0x30BadC453d20b520E0Ed98fce6Ba1AC5876cF1e5",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRX",
        base_currency: "USD",
        address: "0x307cCF7cBD17b69A487b9C3dbe483931Cf3E1833",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRY",
        base_currency: "USD",
        address: "0xd78325DcA0F90F0FFe53cCeA1B02Bb12E1bf8FdB",
    },
    ChainlinkPriceFeed {
        token_symbol: "TSLA",
        base_currency: "USD",
        address: "0x567E67f456c7453c583B6eFA6F18452cDee1F5a8",
    },
    ChainlinkPriceFeed {
        token_symbol: "TUSD",
        base_currency: "USD",
        address: "0x7C5D415B64312D38c56B54358449d0a4058339d2",
    },
    ChainlinkPriceFeed {
        token_symbol: "UMA",
        base_currency: "USD",
        address: "0x33D9B1BAaDcF4b26ab6F8E83e9cb8a611B2B3956",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "ETH",
        address: "0x162d8c5bF15eB6BEe003a1ffc4049C92114bc931",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "USD",
        address: "0xdf0Fb4e4F928d2dCB76f438575fDD8682386e13C",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "ETH",
        address: "0xefb7e6be8356cCc6827799B6A7348eE674A80EaE",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "USD",
        address: "0xfE4A8cc5b5B2366C1B58Bea3858e81843581b2F7",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "ETH",
        address: "0xf9d5AAC6E5572AEFa6bd64108ff86a222F69B64d",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "USD",
        address: "0x0A6513e40db6EB1b165753AD52E80663aeA50545",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDe",
        base_currency: "USD",
        address: "0x5c3890e86f3E7Ed7F5390532De147953580f1605",
    },
    ChainlinkPriceFeed {
        token_symbol: "WBTC",
        base_currency: "ETH",
        address: "0xA338e0492B2F944E9F8C0653D3AD1484f2657a3",
    },
    ChainlinkPriceFeed {
        token_symbol: "WBTC",
        base_currency: "USD",
        address: "0xDE31F8bFBD8c84b5360CFACCa3539B938dd78ae6",
    },
    ChainlinkPriceFeed {
        token_symbol: "WOO",
        base_currency: "USD",
        address: "0x6a99EC84819FB7007dd5D032068742604E755c56",
    },
    ChainlinkPriceFeed {
        token_symbol: "WSTETH",
        base_currency: "ETH",
        address: "0x10f964234cae09cB6a9854B56FF7D4F38Cda5E6a",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAG",
        base_currency: "USD",
        address: "0x461c7B8D370a240DdB46B402748381C3210136b3",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAU",
        base_currency: "USD",
        address: "0x0C466540B2ee1a31b441671eac0ca886e051E410",
    },
    ChainlinkPriceFeed {
        token_symbol: "XLM",
        base_currency: "USD",
        address: "0x692AE5510cA9070095A496dbcFBCDA99D4024Cd9",
    },
    ChainlinkPriceFeed {
        token_symbol: "XMR",
        base_currency: "USD",
        address: "0xBE6FB0AB6302B693368D0E9001fAF77ecc6571db",
    },
    ChainlinkPriceFeed {
        token_symbol: "XPT",
        base_currency: "USD",
        address: "0x76631863c2ae7367aF8f37Cd10d251DA7f1DE186",
    },
    ChainlinkPriceFeed {
        token_symbol: "XRP",
        base_currency: "USD",
        address: "0x785ba89291f676b5386652eB12b30cF361020694",
    },
    ChainlinkPriceFeed {
        token_symbol: "XTZ",
        base_currency: "USD",
        address: "0x691e26AB58ff05800E028b0876A41B720b26FC65",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "ETH",
        address: "0x9896A1eA7A00F5f32Ab131eBbeE07487B0af31D0",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "USD",
        address: "",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZEC",
        base_currency: "USD",
        address: "0xBC08c639e579a391C4228F20d0C29d0690092DF0",
    },
    ChainlinkPriceFeed {
        token_symbol: "ibBTC PricePerShare",
        base_currency: "",
        address: "0xc3E676E68dB28c9Fb2199f25B60560723237cc76",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USD",
        address: "0xe71A04ACe06e2667c7e1c802E998CBf44A6bf53C",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USDe Exchange Rate",
        address: "0xd1d85Bf11c28F878EE75BdC39387a2a15E0390b7",
    },
    ChainlinkPriceFeed {
        token_symbol: "wstETH-stETH Exchange Rate",
        base_currency: "",
        address: "0x3Ea1eC855fBda8bA0396975eC260AD2e9B2Bc01c",
    },
];

pub static ARBITRUM_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
    ChainlinkPriceFeed {
        token_symbol: "1INCH",
        base_currency: "USD",
        address: "0x4bC735Ef24bf286983024CAd5D03f0738865Aaef",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAPL",
        base_currency: "USD",
        address: "0x8d0CC5f38f9E802475f2CFf4F9fc7000C2E1557c",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "USD",
        address: "0xaD1d5344AaDE45F43E596773Bcc4c423EAbdD034",
    },
    ChainlinkPriceFeed {
        token_symbol: "ADA",
        base_currency: "USD",
        address: "0xD9f615A9b820225edbA2d821c4A696a0924051c6",
    },
    ChainlinkPriceFeed {
        token_symbol: "AMZN",
        base_currency: "USD",
        address: "0xd6a77691f071E98Df7217BED98f38ae6d2313EBA",
    },
    ChainlinkPriceFeed {
        token_symbol: "APE",
        base_currency: "USD",
        address: "0x221912ce795669f628c51c69b7d0873eDA9C03bB",
    },
    ChainlinkPriceFeed {
        token_symbol: "ARB / USD",
        base_currency: "USD",
        address: "0xb2A824043730FE05F3DA2efaFa1CBbe83fa548D6",
    },
    ChainlinkPriceFeed {
        token_symbol: "ASTR",
        base_currency: "USD",
        address: "0x70E48a135F76bA31B47FE944e769E052A8FeB849",
    },
    ChainlinkPriceFeed {
        token_symbol: "ATOM",
        base_currency: "USD",
        address: "0xCDA67618e51762235eacA373894F0C79256768fa",
    },
    ChainlinkPriceFeed {
        token_symbol: "AUD",
        base_currency: "USD",
        address: "0x9854e9a850e7C354c1de177eA953a6b1fba8Fc22",
    },
    ChainlinkPriceFeed {
        token_symbol: "AVAX",
        base_currency: "USD",
        address: "0x8bf61728eeDCE2F32c456454d87B5d6eD6150208",
    },
    ChainlinkPriceFeed {
        token_symbol: "AXL",
        base_currency: "USD",
        address: "0x84e8237CC1418Ea1B4A1e0C3e7F48c3A5fbC81AF",
    },
    ChainlinkPriceFeed {
        token_symbol: "AXS",
        base_currency: "USD",
        address: "0x5B58aA6E0651Ae311864876A55411F481aD86080",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAL",
        base_currency: "USD",
        address: "0xBE5eA816870D11239c543F84b71439511D70B94f",
    },
    ChainlinkPriceFeed {
        token_symbol: "BNB",
        base_currency: "USD",
        address: "0x6970460aabF80C5BE983C6b74e5D06dEDCA95D4A",
    },
    ChainlinkPriceFeed {
        token_symbol: "BRL",
        base_currency: "USD",
        address: "0x04b7384473A2aDF1903E3a98aCAc5D62ba8C2702",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "ETH",
        address: "0xc5a90A6d7e4Af242dA238FFe279e9f2BA0c64B2e",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "USD",
        address: "0x6ce185860a4963106506C203335A2910413708e9",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC-USD Total Marketcap",
        base_currency: "USD",
        address: "0x7519bCA20e21725557Bb98d9032124f8885a26C2",
    },
    ChainlinkPriceFeed {
        token_symbol: "CAD",
        base_currency: "USD",
        address: "0xf6DA27749484843c4F02f5Ad1378ceE723dD61d4",
    },
    ChainlinkPriceFeed {
        token_symbol: "CAKE",
        base_currency: "USD",
        address: "0x256654437f1ADA8057684b18d742eFD14034C400",
    },
    ChainlinkPriceFeed {
        token_symbol: "CBETH",
        base_currency: "ETH",
        address: "0xa668682974E3f121185a3cD94f00322beC674275",
    },
    ChainlinkPriceFeed {
        token_symbol: "CHF",
        base_currency: "USD",
        address: "0xe32AccC8c4eC03F6E75bd3621BfC9Fbb234E1FC3",
    },
    ChainlinkPriceFeed {
        token_symbol: "CNY",
        base_currency: "USD",
        address: "0xcC3370Bde6AFE51e1205a5038947b9836371eCCb",
    },
    ChainlinkPriceFeed {
        token_symbol: "COIN",
        base_currency: "USD",
        address: "0x950DC95D4E537A14283059bADC2734977C454498",
    },
    ChainlinkPriceFeed {
        token_symbol: "COMP",
        base_currency: "USD",
        address: "0xe7C53FFd03Eb6ceF7d208bC4C13446c76d1E5884",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "USD",
        address: "0xaebDA2c976cfd1eE1977Eac079B4382acb849325",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRVUSD",
        base_currency: "USD",
        address: "0x0a32255dd4BB6177C994bAAc73E0606fDD568f66",
    },
    ChainlinkPriceFeed {
        token_symbol: "CVX",
        base_currency: "USD",
        address: "0x851175a919f36c8e30197c09a9A49dA932c2CC00",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "USD",
        address: "0xc5C8E77B397E531B8EC06BFb0048328B30E9eCfB",
    },
    ChainlinkPriceFeed {
        token_symbol: "DODO",
        base_currency: "USD",
        address: "0xA33a06c119EC08F92735F9ccA37e07Af08C4f281",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOGE",
        base_currency: "USD",
        address: "0x9A7FB1b3950837a8D9b40517626E11D4127C098C",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOT",
        base_currency: "USD",
        address: "0xa6bC5bAF2000424e90434bA7104ee399dEe80DEc",
    },
    ChainlinkPriceFeed {
        token_symbol: "DPI",
        base_currency: "USD",
        address: "0x1e431E56118bE414bD91f6392414ad3833d21B0D",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH / ",
        base_currency: "USD",
        address: "0x639Fe6ab55C921f74e7fac1ee960C0B6293ba612",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH-USD Total Marketcap",
        base_currency: "USD",
        address: "0xB1f70A229FE7cceD0428245db8B1f6C48c7Ea82a",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETHx",
        base_currency: "ETH",
        address: "0xB4AC4078DDA43d0eB6Bb9e08b8C12A73f9FEAA7d",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETHx",
        base_currency: "ETH",
        address: "0x1f5C0C2CD2e9Ad1eE475660AF0bBa27aE7d87f5e",
    },
    ChainlinkPriceFeed {
        token_symbol: "EUR",
        base_currency: "USD",
        address: "0xA14d53bC1F1c0F31B4aA3BD109344E5009051a84",
    },
    ChainlinkPriceFeed {
        token_symbol: "FRAX",
        base_currency: "USD",
        address: "0x0809E3d38d1B4214958faf06D8b1B1a2b73f2ab8",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTM",
        base_currency: "USD",
        address: "0xFeaC1A3936514746e70170c0f539e70b23d36F19",
    },
    ChainlinkPriceFeed {
        token_symbol: "FXS",
        base_currency: "USD",
        address: "0x36a121448D74Fa81450c992A1a44B9b7377CD3a5",
    },
    ChainlinkPriceFeed {
        token_symbol: "GBP",
        base_currency: "USD",
        address: "0x9C4424Fd84C6661F97D8d6b3fc3C1aAc2BeDd137",
    },
    ChainlinkPriceFeed {
        token_symbol: "GHO",
        base_currency: "USD",
        address: "0x3c786e934F23375Ca345C9b8D5aD54838796E8e7",
    },
    ChainlinkPriceFeed {
        token_symbol: "GMX",
        base_currency: "USD",
        address: "0xDB98056FecFff59D032aB628337A4887110df3dB",
    },
    ChainlinkPriceFeed {
        token_symbol: "GNS",
        base_currency: "USD",
        address: "0xE89E98CE4E19071E59Ed4780E0598b541CE76486",
    },
    ChainlinkPriceFeed {
        token_symbol: "GOOGL",
        base_currency: "USD",
        address: "0x1D1a83331e9D255EB1Aaf75026B60dFD00A252ba",
    },
    ChainlinkPriceFeed {
        token_symbol: "GRT",
        base_currency: "USD",
        address: "0x0F38D86FceF4955B705F35c9e41d1A16e0637c73",
    },
    ChainlinkPriceFeed {
        token_symbol: "IOTX",
        base_currency: "USD",
        address: "0x484A1b29ED1Ea038dBd75D7c7293714343363122",
    },
    ChainlinkPriceFeed {
        token_symbol: "JOE",
        base_currency: "USD",
        address: "0x04180965a782E487d0632013ABa488A472243542",
    },
    ChainlinkPriceFeed {
        token_symbol: "JPY",
        base_currency: "USD",
        address: "0x3dD6e51CB9caE717d5a8778CF79A04029f9cFDF8",
    },
    ChainlinkPriceFeed {
        token_symbol: "KNC",
        base_currency: "USD",
        address: "0xbF539d4c2106dd4D9AB6D56aed3d9023529Db145",
    },
    ChainlinkPriceFeed {
        token_symbol: "KRW",
        base_currency: "USD",
        address: "0x85bb02E0Ae286600d1c68Bb6Ce22Cc998d411916",
    },
    ChainlinkPriceFeed {
        token_symbol: "LDO",
        base_currency: "USD",
        address: "0xA43A34030088E6510FecCFb77E88ee5e7ed0fE64",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "ETH",
        address: "0xb7c8Fb1dB45007F98A68Da0588e1AA524C317f27",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "USD",
        address: "0x86E53CF1B870786351Da77A57575e79CB55812CB",
    },
    ChainlinkPriceFeed {
        token_symbol: "LTC",
        base_currency: "USD",
        address: "0x5698690a7B7B84F6aa985ef7690A8A7288FBc9c8",
    },
    ChainlinkPriceFeed {
        token_symbol: "LUSD",
        base_currency: "USD",
        address: "0x0411D28c94d85A36bC72Cb0f875dfA8371D8fFfF",
    },
    ChainlinkPriceFeed {
        token_symbol: "MAGIC",
        base_currency: "USD",
        address: "0x47E55cCec6582838E173f252D08Afd8116c2202d",
    },
    ChainlinkPriceFeed {
        token_symbol: "MATIC",
        base_currency: "USD",
        address: "0x52099D4523531f678Dfc568a7B1e5038aadcE1d6",
    },
    ChainlinkPriceFeed {
        token_symbol: "META",
        base_currency: "USD",
        address: "0xcd1bd86fDc33080DCF1b5715B6FCe04eC6F85845",
    },
    ChainlinkPriceFeed {
        token_symbol: "MKR",
        base_currency: "USD",
        address: "0xdE9f0894670c4EFcacF370426F10C3AD2Cdf147e",
    },
    ChainlinkPriceFeed {
        token_symbol: "MLN",
        base_currency: "USD",
        address: "0xD07de6e37A011CCAfD375d7eb130205E0fa24d69",
    },
    ChainlinkPriceFeed {
        token_symbol: "MSFT",
        base_currency: "USD",
        address: "0xDde33fb9F21739602806580bdd73BAd831DcA867",
    },
    ChainlinkPriceFeed {
        token_symbol: "MVI",
        base_currency: "USD",
        address: "0x87B679C03e9672Af516a9E08085e1F4FA1722A3D",
    },
    ChainlinkPriceFeed {
        token_symbol: "NEAR",
        base_currency: "USD",
        address: "0xBF5C3fB2633e924598A46B9D07a174a9DBcF57C0",
    },
    ChainlinkPriceFeed {
        token_symbol: "NVDA",
        base_currency: "USD",
        address: "0x4881A4418b5F2460B21d6F08CD5aA0678a7f262F",
    },
    ChainlinkPriceFeed {
        token_symbol: "OHM Index",
        base_currency: "",
        address: "0x48C4721354A3B29D80EF03C65E6644A37338a0B1",
    },
    ChainlinkPriceFeed {
        token_symbol: "OP",
        base_currency: "USD",
        address: "0x205aaD468a11fd5D34fA7211bC6Bad5b3deB9b98",
    },
    ChainlinkPriceFeed {
        token_symbol: "PAXG",
        base_currency: "USD",
        address: "0x2BA975D4D7922cD264267Af16F3bD177F206FE3c",
    },
    ChainlinkPriceFeed {
        token_symbol: "PENDLE",
        base_currency: "USD",
        address: "0x66853E19d73c0F9301fe099c324A1E9726953433",
    },
    ChainlinkPriceFeed {
        token_symbol: "PEPE",
        base_currency: "USD",
        address: "0x02DEd5a7EDDA750E3Eb240b54437a54d57b74dBE",
    },
    ChainlinkPriceFeed {
        token_symbol: "PHP",
        base_currency: "USD",
        address: "0xfF82AAF635645fD0bcc7b619C3F28004cDb58574",
    },
    ChainlinkPriceFeed {
        token_symbol: "RDNT",
        base_currency: "USD",
        address: "0x20d0Fcab0ECFD078B036b6CAf1FaC69A6453b352",
    },
    ChainlinkPriceFeed {
        token_symbol: "RETH",
        base_currency: "ETH",
        address: "0xD6aB2298946840262FcC278fF31516D39fF611eF",
    },
    ChainlinkPriceFeed {
        token_symbol: "RPL / ",
        base_currency: "USD",
        address: "0xF0b7159BbFc341Cc41E7Cb182216F62c6d40533D",
    },
    ChainlinkPriceFeed {
        token_symbol: "RSETH",
        base_currency: "ETH",
        address: "0xb0EA543f9F8d4B818550365d13F66Da747e1476A",
    },
    ChainlinkPriceFeed {
        token_symbol: "RSR",
        base_currency: "USD",
        address: "0xcfF9349ec6d027f20fC9360117fef4a1Ad38B488",
    },
    ChainlinkPriceFeed {
        token_symbol: "SEK",
        base_currency: "USD",
        address: "0xdE89a55d04DEd40A410877ab87d4F567ee66a1f0",
    },
    ChainlinkPriceFeed {
        token_symbol: "SGD",
        base_currency: "USD",
        address: "0xF0d38324d1F86a176aC727A4b0c43c9F9d9c5EB1",
    },
    ChainlinkPriceFeed {
        token_symbol: "SNX / ",
        base_currency: "USD",
        address: "0x054296f0D036b95531B4E14aFB578B80CFb41252",
    },
    ChainlinkPriceFeed {
        token_symbol: "SOL",
        base_currency: "USD",
        address: "0x24ceA4b8ce57cdA5058b924B9B9987992450590c",
    },
    ChainlinkPriceFeed {
        token_symbol: "SPELL",
        base_currency: "USD",
        address: "0x383b3624478124697BEF675F07cA37570b73992f",
    },
    ChainlinkPriceFeed {
        token_symbol: "SPY",
        base_currency: "USD",
        address: "0x46306F3795342117721D8DEd50fbcF6DF2b3cc10",
    },
    ChainlinkPriceFeed {
        token_symbol: "STETH",
        base_currency: "ETH",
        address: "0xded2c52b75B24732e9107377B7Ba93eC1fFa4BAf",
    },
    ChainlinkPriceFeed {
        token_symbol: "STETH",
        base_currency: "USD",
        address: "0x07C5b924399cc23c24a95c8743DE4006a32b7f2a",
    },
    ChainlinkPriceFeed {
        token_symbol: "STG",
        base_currency: "USD",
        address: "0xe74d69E233faB0d8F48921f2D93aDfDe44cEb3B7",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "USD",
        address: "0xb2A8BA74cbca38508BA1632761b56C897060147C",
    },
    ChainlinkPriceFeed {
        token_symbol: "SWETH",
        base_currency: "ETH",
        address: "0x05Bc6e5Fb110589bb366A3Cd7CdBe143EeBA2168",
    },
    ChainlinkPriceFeed {
        token_symbol: "StaFi Staked ETH rETH",
        base_currency: "ETH",
        address: "0x052d4200b624b07262F574af26C71A6553996Ab5",
    },
    ChainlinkPriceFeed {
        token_symbol: "TAO",
        base_currency: "USD",
        address: "0x6aCcBB82aF71B8a576B4C05D4aF92A83A035B991",
    },
    ChainlinkPriceFeed {
        token_symbol: "TBTC",
        base_currency: "USD",
        address: "0xE808488e8627F6531bA79a13A9E0271B39abEb1C",
    },
    ChainlinkPriceFeed {
        token_symbol: "TIA",
        base_currency: "USD",
        address: "0x4096b9bfB4c34497B7a3939D4f629cf65EBf5634",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRY",
        base_currency: "USD",
        address: "0xE8f8AfE4b56c6C421F691bfAc225cE61b2C7CD05",
    },
    ChainlinkPriceFeed {
        token_symbol: "TSLA",
        base_currency: "USD",
        address: "0x3609baAa0a9b1f0FE4d6CC01884585d0e191C3E3",
    },
    ChainlinkPriceFeed {
        token_symbol: "TUSD",
        base_currency: "USD",
        address: "0x6fAbee62266Da6686EE2744C6f15bb8352d2f28D",
    },
    ChainlinkPriceFeed {
        token_symbol: "Total Marketcap USD",
        base_currency: "USD",
        address: "0x4763b84cdBc5211B9e0a57D5E39af3B3b2440012",
    },
    ChainlinkPriceFeed {
        token_symbol: "ULTI",
        base_currency: "USD",
        address: "0x8883045300Eaf3b1Bb1b3b17F9B4d70EfF50212a",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "USD",
        address: "0x9C917083fDb403ab5ADbEC26Ee294f6EcAda2720",
    },
    ChainlinkPriceFeed {
        token_symbol: "USD+",
        base_currency: "USD",
        address: "0x6548a81E640C000150e06AB413fB3F772682e9c5",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "USD",
        address: "0x50834F3163758fcC1Df9973b6e91f0F0F0434aD3",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDD",
        base_currency: "USD",
        address: "0x4Ee1f9ec1048979930aC832a3C1d18a0b4955a02",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "USD",
        address: "0x3f3f5dF88dC9F13eac63DF89EC16ef6e7E25DdE7",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDV",
        base_currency: "USD",
        address: "0x7Fa028B87e73deb66DcFf9Fa40f4C7C6Dd2Fd254",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDe",
        base_currency: "USD",
        address: "0x88AC7Bca36567525A866138F03a6F6844868E0Bc",
    },
    ChainlinkPriceFeed {
        token_symbol: "WBTC",
        base_currency: "BTC",
        address: "0x0017abAc5b6f291F9164e35B1234CA1D697f9CF4",
    },
    ChainlinkPriceFeed {
        token_symbol: "WBTC",
        base_currency: "USD",
        address: "0xd0C7101eACbB49F3deCcCc166d238410D6D46d57",
    },
    ChainlinkPriceFeed {
        token_symbol: "WIF",
        base_currency: "USD",
        address: "0xF7Ee427318d2Bd0EEd3c63382D0d52Ad8A68f90D",
    },
    ChainlinkPriceFeed {
        token_symbol: "WOO",
        base_currency: "USD",
        address: "0x5e2b5C5C07cCA3437c4D724225Bb42c7E55d1597",
    },
    ChainlinkPriceFeed {
        token_symbol: "WSTETH",
        base_currency: "ETH",
        address: "0xb523AE262D20A936BC152e6023996e46FDC2A95D",
    },
    ChainlinkPriceFeed {
        token_symbol: "WTI",
        base_currency: "USD",
        address: "0x594b919AD828e693B935705c3F816221729E7AE8",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAG",
        base_currency: "USD",
        address: "0xC56765f04B248394CF1619D20dB8082Edbfa75b1",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAI",
        base_currency: "USD",
        address: "0x806c532D543352e7C344ba6C7F3F00Bfbd309Af1",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAU",
        base_currency: "USD",
        address: "0x1F954Dc24a49708C26E0C1777f16750B5C6d5a2c",
    },
    ChainlinkPriceFeed {
        token_symbol: "XRP",
        base_currency: "USD",
        address: "0xB4AD57B52aB9141de9926a3e0C8dc6264c2ef205",
    },
    ChainlinkPriceFeed {
        token_symbol: "XVS",
        base_currency: "USD",
        address: "0x300b0990Ba191a1AeBef6e5Ed8B5B308C0B2d0c9",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "USD",
        address: "0x745Ab5b69E01E2BE1104Ca84937Bb71f96f5fB21",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZAR",
        base_currency: "USD",
        address: "0xA9cC9B5Ea2584239365Ea6b985868D121CB7Aea6",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZRO",
        base_currency: "USD",
        address: "0x1940fEd49cDBC397941f2D336eb4994D599e568B",
    },
    ChainlinkPriceFeed {
        token_symbol: "ankrETH",
        base_currency: "ETH",
        address: "0x5Fb73f7Af8a29297953d3611422826039338E5b4",
    },
    ChainlinkPriceFeed {
        token_symbol: "apxETH",
        base_currency: "pxETH",
        address: "0x41f8459f911658e9C627c5D1d2Fb18c7dbc8ceBe",
    },
    ChainlinkPriceFeed {
        token_symbol: "cbETH",
        base_currency: "ETH",
        address: "0x0518673439245BB95A58688Bc31cd513F3D5bDd6",
    },
    ChainlinkPriceFeed {
        token_symbol: "egETH",
        base_currency: "ETH",
        address: "0xD3631AC9D81eD560D61957a55E9c992cdE497eb6",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0x11E1836bFF2ce9d6A5bec9cA79dc998210f3886d",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0x989a480b6054389075CBCdC385C18CfB6FC08186",
    },
    ChainlinkPriceFeed {
        token_symbol: "frxETH",
        base_currency: "ETH",
        address: "0x5C3e80763862CB777Aa07BDDBcCE0123104e1c34",
    },
    ChainlinkPriceFeed {
        token_symbol: "frxETH",
        base_currency: "ETH",
        address: "0x1bD872f3A606471787B1a304cE0356e4e87Af930",
    },
    ChainlinkPriceFeed {
        token_symbol: "gmARB",
        base_currency: "USD",
        address: "0x5d046567b97B0d7322F2402e3b34Bf789cE329f5",
    },
    ChainlinkPriceFeed {
        token_symbol: "gmBTC",
        base_currency: "USD",
        address: "0x395D5c5D552Df670dc4B2B1cef0c4EABfFba492f",
    },
    ChainlinkPriceFeed {
        token_symbol: "gmETH",
        base_currency: "USD",
        address: "0xfB3264D1129824933a52374c2C1696F4470D041e",
    },
    ChainlinkPriceFeed {
        token_symbol: "inETH",
        base_currency: "ETH",
        address: "0x3C5C5329b028E674F7a124b18527B94BE74A66cF",
    },
    ChainlinkPriceFeed {
        token_symbol: "instETH",
        base_currency: "ETH",
        address: "0x4050bD8263771f8BBded08C299BD944488a91AaD",
    },
    ChainlinkPriceFeed {
        token_symbol: "osETH",
        base_currency: "ETH",
        address: "0xB4102D5E72c402D537C9f024F4bd9c3709FE200d",
    },
    ChainlinkPriceFeed {
        token_symbol: "rETH",
        base_currency: "ETH",
        address: "0xF3272CAfe65b190e76caAF483db13424a3e23dD2",
    },
    ChainlinkPriceFeed {
        token_symbol: "rswETH",
        base_currency: "ETH",
        address: "0xC3534C27E3DE2ae861EB38889a1c8dCfEa4Cb39d",
    },
    ChainlinkPriceFeed {
        token_symbol: "rswETH",
        base_currency: "ETH",
        address: "0x683989a7A6424122678164Ad26736bA484055B35",
    },
    ChainlinkPriceFeed {
        token_symbol: "sFRAX",
        base_currency: "FRAX",
        address: "0x03e4054B11ad01915257bE53Af03A32Abf7837b9",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USD",
        address: "0xf2215b9c35b1697B5f47e407c917a40D055E68d7",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USDe",
        address: "0x605EA726F0259a30db5b7c9ef39Df9fE78665C44",
    },
    ChainlinkPriceFeed {
        token_symbol: "sfrxETH",
        base_currency: "frxETH",
        address: "0x98E5a52fB741347199C08a7a3fcF017364284431",
    },
    ChainlinkPriceFeed {
        token_symbol: "swETH",
        base_currency: "ETH",
        address: "0xEcD471ef663eCEFFC7D731a0C7e51007433e6d6e",
    },
    ChainlinkPriceFeed {
        token_symbol: "uniETH",
        base_currency: "ETH",
        address: "0xF18BE32dB91591A7256d738C166FA195a17457DA",
    },
    ChainlinkPriceFeed {
        token_symbol: "wOETH",
        base_currency: "OETH",
        address: "0x03a1f4b19aaeA6e68f0f104dc4346dA3E942cC45",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "ETH",
        address: "0xE141425bc1594b8039De6390db1cDaf4397EA22b",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "eETH",
        address: "0x20bAe7e1De9c596f5F7615aeaa1342Ba99294e12",
    },
    ChainlinkPriceFeed {
        token_symbol: "wrsETH",
        base_currency: "rsETH",
        address: "0x8f1dF6D7F2db73eECE86a18b4381F4707b918FB1",
    },
    ChainlinkPriceFeed {
        token_symbol: "wstETH",
        base_currency: "stETH",
        address: "0xB1552C5e96B312d0Bf8b554186F846C40614a540",
    },
    ChainlinkPriceFeed {
        token_symbol: "ynETH",
        base_currency: "ETH",
        address: "0xc2430cD1214F8452f4040473b7587195f9c565bD",
    },
];

pub static OPTIMISM_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "USD",
        address: "0x338ed6787f463394D24813b297401B9F05a8C9d1",
    },
    ChainlinkPriceFeed {
        token_symbol: "ADA",
        base_currency: "USD",
        address: "0x43dEa17DeE1ca50c6266acb59b32659E44D3ee5D",
    },
    ChainlinkPriceFeed {
        token_symbol: "ALGO",
        base_currency: "USD",
        address: "0xBf5384854988939729E8B76b8AeCe7d8D930F9f3",
    },
    ChainlinkPriceFeed {
        token_symbol: "APE",
        base_currency: "USD",
        address: "0x89178957E9bD07934d7792fFc0CF39f11c8C2B1F",
    },
    ChainlinkPriceFeed {
        token_symbol: "APT",
        base_currency: "USD",
        address: "0x48f2EcF0Bd180239AEF474a9da945F2e2d41daA3",
    },
    ChainlinkPriceFeed {
        token_symbol: "ARB",
        base_currency: "USD",
        address: "0x8f14546d0B960793180ee355B73fA55041a4a356",
    },
    ChainlinkPriceFeed {
        token_symbol: "ATOM",
        base_currency: "USD",
        address: "0xEF89db2eA46B4aD4E333466B6A486b809e613F39",
    },
    ChainlinkPriceFeed {
        token_symbol: "AUD",
        base_currency: "USD",
        address: "0x39be70E93D2D285C9E71be7f70FC5a45A7777B14",
    },
    ChainlinkPriceFeed {
        token_symbol: "AVAX",
        base_currency: "USD",
        address: "0x5087Dc69Fd3907a016BD42B38022F7f024140727",
    },
    ChainlinkPriceFeed {
        token_symbol: "AXS",
        base_currency: "USD",
        address: "0x805a61D54bb686e57F02D1EC96A1491C7aF40893",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAL",
        base_currency: "USD",
        address: "0x30D9d31C1ac29Bc2c2c312c1bCa9F8b3D60e2376",
    },
    ChainlinkPriceFeed {
        token_symbol: "BCH",
        base_currency: "USD",
        address: "0x33E047119359161288bcB143e0C15467C7151d4c",
    },
    ChainlinkPriceFeed {
        token_symbol: "BLUR",
        base_currency: "USD",
        address: "0x517C2557c29F7c53Aa5F97a1DAE465E0d5C174AA",
    },
    ChainlinkPriceFeed {
        token_symbol: "BNB",
        base_currency: "USD",
        address: "0xD38579f7cBD14c22cF1997575eA8eF7bfe62ca2c",
    },
    ChainlinkPriceFeed {
        token_symbol: "BONK",
        base_currency: "USD",
        address: "0xec236454209A76a6deCdf5C1183aE2Eb5e82a829",
    },
    ChainlinkPriceFeed {
        token_symbol: "BRL",
        base_currency: "USD",
        address: "0xB22900D4D0CEa5DB0B3bb08565a9f0f4a831D32C",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "USD",
        address: "0xD702DD976Fb76Fffc2D3963D037dfDae5b04E593",
    },
    ChainlinkPriceFeed {
        token_symbol: "CBETH",
        base_currency: "ETH",
        address: "0x138b809B8472fF09Cd3E075E6EcbB2e42D41d870",
    },
    ChainlinkPriceFeed {
        token_symbol: "COMP",
        base_currency: "USD",
        address: "0xe1011160d78a80E2eEBD60C228EEf7af4Dfcd4d7",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "USD",
        address: "0xbD92C6c284271c227a1e0bF1786F468b539f51D9",
    },
    ChainlinkPriceFeed {
        token_symbol: "CVX",
        base_currency: "USD",
        address: "0x955b05dD4573dDFAfB47cb78db16B1Fa127E6e71",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "USD",
        address: "0x8dBa75e83DA73cc766A7e5a0ee71F656BAb470d6",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOGE",
        base_currency: "USD",
        address: "0xC6066533917f034Cf610c08e1fe5e9c7eADe0f54",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOT",
        base_currency: "USD",
        address: "0x28e67BAeEB5dE7A788f3Dde6CF6ee491369Bb3Fa",
    },
    ChainlinkPriceFeed {
        token_symbol: "DYDX",
        base_currency: "USD",
        address: "0xee35A95c9a064491531493D8b380bC40A4CCd0Da",
    },
    ChainlinkPriceFeed {
        token_symbol: "EOS",
        base_currency: "USD",
        address: "0x8E8E6C8c4942e4963C682fF54A0d058458393DCC",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETC",
        base_currency: "USD",
        address: "0xb7B9A39CC63f856b90B364911CC324dC46aC1770",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "BTC",
        address: "0xe4b9bcD7d0AA917f19019165EB89BdbbF36d2cBe",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "USD",
        address: "0x13e3Ee699D1909E989722E753853AE30b17e08c5",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETHx",
        base_currency: "ETH",
        address: "0x4Fe3caF5752AD3EEE3BfC0Bb4D07069E569bc66C",
    },
    ChainlinkPriceFeed {
        token_symbol: "EUR",
        base_currency: "USD",
        address: "0x3626369857A10CcC6cc3A6e4f5C2f5984a519F20",
    },
    ChainlinkPriceFeed {
        token_symbol: "FIL",
        base_currency: "USD",
        address: "0x66F61FEe824c1dF059BccCC5F21ca39e083EefDf",
    },
    ChainlinkPriceFeed {
        token_symbol: "FLOW",
        base_currency: "USD",
        address: "0x2fF1EB7D0ceC35959F0248E9354c3248c6683D9b",
    },
    ChainlinkPriceFeed {
        token_symbol: "FRAX",
        base_currency: "USD",
        address: "0xc7D132BeCAbE7Dcc4204841F33bae45841e41D9C",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTM",
        base_currency: "USD",
        address: "0xc19d58652d6BfC6Db6FB3691eDA6Aa7f3379E4E9",
    },
    ChainlinkPriceFeed {
        token_symbol: "FXS",
        base_currency: "USD",
        address: "0xB9B16330671067B1b062B9aC2eFd2dB75F03436E",
    },
    ChainlinkPriceFeed {
        token_symbol: "GBP",
        base_currency: "USD",
        address: "0x540D48C01F946e729174517E013Ad0bdaE5F08C0",
    },
    ChainlinkPriceFeed {
        token_symbol: "GMX",
        base_currency: "USD",
        address: "0x62f42f70ba85De1086476bB6BADE926d0E0b8a4C",
    },
    ChainlinkPriceFeed {
        token_symbol: "GRT",
        base_currency: "USD",
        address: "0xfa042d5F474d7A39454C594CCfE014Ea011495f2",
    },
    ChainlinkPriceFeed {
        token_symbol: "ICP",
        base_currency: "USD",
        address: "0xe98290265E4aE3758503a03e937F381A2A7aFB57",
    },
    ChainlinkPriceFeed {
        token_symbol: "IMX",
        base_currency: "USD",
        address: "0x26Fce884555FAe5F0E4701cc976FE8D8bB111A38",
    },
    ChainlinkPriceFeed {
        token_symbol: "INJ",
        base_currency: "USD",
        address: "0x90CC16F5493894eff84a5Fedd1dcE297d174fEEf",
    },
    ChainlinkPriceFeed {
        token_symbol: "INR",
        base_currency: "USD",
        address: "0x5535e67d8f99c8ebe961E1Fc1F6DDAE96FEC82C9",
    },
    ChainlinkPriceFeed {
        token_symbol: "JPY",
        base_currency: "USD",
        address: "0x536944c3A71FEb7c1E5C66Ee37d1a148d8D8f619",
    },
    ChainlinkPriceFeed {
        token_symbol: "JTO",
        base_currency: "USD",
        address: "0xFC3b7bd4368b2919f67E437f8c6Ca42C7FD55dd5",
    },
    ChainlinkPriceFeed {
        token_symbol: "JUP",
        base_currency: "USD",
        address: "0x5eb9F7baCd59C886fBD9aa2C0a891223482a1ed4",
    },
    ChainlinkPriceFeed {
        token_symbol: "KNC",
        base_currency: "USD",
        address: "0xCB24d22aF35986aC1feb8874AdBbDF68f6dC2e96",
    },
    ChainlinkPriceFeed {
        token_symbol: "LDO",
        base_currency: "USD",
        address: "0x221618871470f78D8a3391d35B77dFb3C0fbc383",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "ETH",
        address: "0x464A1515ADc20de946f8d0DEB99cead8CEAE310d",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "USD",
        address: "0xCc232dcFAAE6354cE191Bd574108c1aD03f86450",
    },
    ChainlinkPriceFeed {
        token_symbol: "LTC",
        base_currency: "USD",
        address: "0x45954efBD01f5A12428A09E4C38b8434C3dD4Ac3",
    },
    ChainlinkPriceFeed {
        token_symbol: "LUSD",
        base_currency: "USD",
        address: "0x9dfc79Aaeb5bb0f96C6e9402671981CdFc424052",
    },
    ChainlinkPriceFeed {
        token_symbol: "MATIC",
        base_currency: "USD",
        address: "0x0ded608AFc23724f614B76955bbd9dFe7dDdc828",
    },
    ChainlinkPriceFeed {
        token_symbol: "MAV",
        base_currency: "USD",
        address: "0x51E06250C8E46c8E5DE41ac8B917a47D706128C2",
    },
    ChainlinkPriceFeed {
        token_symbol: "MEME",
        base_currency: "USD",
        address: "0xC6884869673a6960486FE0f6B0E775A53521e433",
    },
    ChainlinkPriceFeed {
        token_symbol: "MKR",
        base_currency: "USD",
        address: "0x607b417DF51e0E1ed3A12fDb7FC0e8307ED250F3",
    },
    ChainlinkPriceFeed {
        token_symbol: "NEAR",
        base_currency: "USD",
        address: "0xca6fa4b8CB365C02cd3Ba70544EFffe78f63ac82",
    },
    ChainlinkPriceFeed {
        token_symbol: "ONE",
        base_currency: "USD",
        address: "0x7CFB4fac1a2FDB1267F8bc17FADc12804AC13CFE",
    },
    ChainlinkPriceFeed {
        token_symbol: "OP",
        base_currency: "USD",
        address: "0x0D276FC14719f9292D5C1eA2198673d1f4269246",
    },
    ChainlinkPriceFeed {
        token_symbol: "ORDI",
        base_currency: "USD",
        address: "0x30795BeACc0f43920EF1288dB6676B5e205AE288",
    },
    ChainlinkPriceFeed {
        token_symbol: "PENDLE",
        base_currency: "USD",
        address: "0x58F23F80bF389DB1af9e3aA8c59679806749A8a4",
    },
    ChainlinkPriceFeed {
        token_symbol: "PEPE",
        base_currency: "USD",
        address: "0x64Ecf089a6594Be781908D5a26FC8fA6CB08A2C7",
    },
    ChainlinkPriceFeed {
        token_symbol: "PERP",
        base_currency: "USD",
        address: "0xA12CDDd8e986AF9288ab31E58C60e65F2987fB13",
    },
    ChainlinkPriceFeed {
        token_symbol: "PYTH",
        base_currency: "USD",
        address: "0x0838cFe6A97C9CE1611a6Ed17252477a3c71eBEb",
    },
    ChainlinkPriceFeed {
        token_symbol: "RETH",
        base_currency: "ETH",
        address: "0xb429DE60943a8e6DeD356dca2F93Cd31201D9ed0",
    },
    ChainlinkPriceFeed {
        token_symbol: "RPL",
        base_currency: "USD",
        address: "0xADE082c91A6AeCC86fC11704a830e933e1b382eA",
    },
    ChainlinkPriceFeed {
        token_symbol: "RSETH",
        base_currency: "ETH",
        address: "0x03fe94a215E3842deD931769F913d93FF33d0051",
    },
    ChainlinkPriceFeed {
        token_symbol: "RUNE",
        base_currency: "USD",
        address: "0x372cc5e685115A56F14fa7e4716F1294e04c278A",
    },
    ChainlinkPriceFeed {
        token_symbol: "SAND",
        base_currency: "USD",
        address: "0xAE33e077a02071E62d342E449Afd9895b016d541",
    },
    ChainlinkPriceFeed {
        token_symbol: "SEI",
        base_currency: "USD",
        address: "0x6f6cED6B096708C1276056fdBdb7BbDe07Ca462C",
    },
    ChainlinkPriceFeed {
        token_symbol: "SHIB",
        base_currency: "USD",
        address: "0xd1e56e7657C0E0d20c0e11C2B6ae0D90932d5665",
    },
    ChainlinkPriceFeed {
        token_symbol: "SNX",
        base_currency: "USD",
        address: "0x2FCF37343e916eAEd1f1DdaaF84458a359b53877",
    },
    ChainlinkPriceFeed {
        token_symbol: "SOL",
        base_currency: "USD",
        address: "0xC663315f7aF904fbbB0F785c32046dFA03e85270",
    },
    ChainlinkPriceFeed {
        token_symbol: "STETH",
        base_currency: "ETH",
        address: "0x14d2d3a82AeD4019FddDfe07E8bdc485fb0d2249",
    },
    ChainlinkPriceFeed {
        token_symbol: "STETH",
        base_currency: "USD",
        address: "0x41878779a388585509657CE5Fb95a80050502186",
    },
    ChainlinkPriceFeed {
        token_symbol: "STRK",
        base_currency: "USD",
        address: "0x8814dEC83E2862A3792A0D6aDFC48CF76Add1890",
    },
    ChainlinkPriceFeed {
        token_symbol: "STX",
        base_currency: "USD",
        address: "0x602eB777BE29Fbe63349A33306bD73Ff333D02bB",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUI",
        base_currency: "USD",
        address: "0xEaf1a9fe242aa9928faedc6CE7e09aD4875f7133",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSD",
        base_currency: "USD",
        address: "0x7f99817d87baD03ea21E05112Ca799d715730efe",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "USD",
        address: "0x72155D46FD9f03AF1739637F9E7Db8A87C40A730",
    },
    ChainlinkPriceFeed {
        token_symbol: "TBTC",
        base_currency: "USD",
        address: "0x5a61374950D4BFa5a3D4f2CA36FC1d23A92b6f21",
    },
    ChainlinkPriceFeed {
        token_symbol: "TIA",
        base_currency: "USD",
        address: "0xD7bC56BBF8D555936cb5121f38d1d362c586776A",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRX",
        base_currency: "USD",
        address: "0x0E09921cf7801A5aD47B892C8727593275625a9f",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "USD",
        address: "0x11429eE838cC01071402f21C219870cbAc0a59A0",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "USD",
        address: "0x16a9FA2FDa030272Ce99B29CF780dFA30361E0f3",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "USD",
        address: "0xECef79E109e997bCA29c1c0897ec9d7b03647F5E",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDe",
        base_currency: "USD",
        address: "0xEEDF0B095B5dfe75F3881Cb26c19DA209A27463a",
    },
    ChainlinkPriceFeed {
        token_symbol: "VELO",
        base_currency: "USD",
        address: "0x0f2Ed59657e391746C1a097BDa98F2aBb94b1120",
    },
    ChainlinkPriceFeed {
        token_symbol: "WBTC",
        base_currency: "USD",
        address: "0x718A5788b89454aAE3A028AE9c111A29Be6c2a6F",
    },
    ChainlinkPriceFeed {
        token_symbol: "WIF",
        base_currency: "USD",
        address: "0x75c3bF05EeF2c1966D6d5890Def3DbE640627642",
    },
    ChainlinkPriceFeed {
        token_symbol: "WLD",
        base_currency: "USD",
        address: "0x4e1C6B168DCFD7758bC2Ab9d2865f1895813D236",
    },
    ChainlinkPriceFeed {
        token_symbol: "WSTETH",
        base_currency: "ETH",
        address: "0x524299Ab0987a7c4B3c8022a35669DdcdC715a10",
    },
    ChainlinkPriceFeed {
        token_symbol: "WSTETH",
        base_currency: "USD",
        address: "0x698B585CbC4407e2D54aa898B2600B53C68958f7",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAG",
        base_currency: "USD",
        address: "0x290dd71254874f0d4356443607cb8234958DEe49",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAU",
        base_currency: "USD",
        address: "0x8F7bFb42Bf7421c2b34AAD619be4654bFa7B3B8B",
    },
    ChainlinkPriceFeed {
        token_symbol: "XLM",
        base_currency: "USD",
        address: "0x799A346e7dBfa0f66Ad0961259366F93A1ee34C4",
    },
    ChainlinkPriceFeed {
        token_symbol: "XMR",
        base_currency: "USD",
        address: "0x2a8D91686A048E98e6CCF1A89E82f40D14312672",
    },
    ChainlinkPriceFeed {
        token_symbol: "XRP",
        base_currency: "USD",
        address: "0x8788F0DBDa7678244Ac7FF09d963d7696D56A8a0",
    },
    ChainlinkPriceFeed {
        token_symbol: "XTZ",
        base_currency: "USD",
        address: "0xeA2aeD0087A620995Bf609D1bCD76Ea099905138",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "USD",
        address: "0x5cdC797acCBf57EE2363Fed9701262Abc87a232e",
    },
    ChainlinkPriceFeed {
        token_symbol: "ankrETH",
        base_currency: "ETH",
        address: "0x98FB6366E71E9A71C247052d447E928303e17bB6",
    },
    ChainlinkPriceFeed {
        token_symbol: "apxETH",
        base_currency: "pxETH",
        address: "0x21515B1Da412ecdCa071a84f32193eD90D4ddb59",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0xFAD40C0e2BeF93c6a822015863045CAAeAAde4d3",
    },
    ChainlinkPriceFeed {
        token_symbol: "inETH",
        base_currency: "ETH",
        address: "0x5CC26e6798A3b05525076913840aa07b1d65eE00",
    },
    ChainlinkPriceFeed {
        token_symbol: "instETH",
        base_currency: "ETH",
        address: "0x028F7347Df25220DA7D63eB7c29fa10448231489",
    },
    ChainlinkPriceFeed {
        token_symbol: "rETH",
        base_currency: "ETH",
        address: "0x22F3727be377781d1579B7C9222382b21c9d1a8f",
    },
    ChainlinkPriceFeed {
        token_symbol: "rswETH",
        base_currency: "ETH",
        address: "0x90da1D1DB26c0A84164Ff03090e0B31Ad08A137A",
    },
    ChainlinkPriceFeed {
        token_symbol: "sFRAX",
        base_currency: "FRAX",
        address: "0x8f096bFFe37313Ad6bD5B9fF48F9FF6E4E5Cd065",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USD",
        address: "0x05950959B6d876ae0fed1BBe5Caa2d74d8659D59",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USDe",
        address: "0x85342bC62aadef58f029ab6d17D643949e6F073e",
    },
    ChainlinkPriceFeed {
        token_symbol: "sfrxETH",
        base_currency: "frxETH",
        address: "0xd2AdD08d9Cd83720c9296A991ce066BB08265eAc",
    },
    ChainlinkPriceFeed {
        token_symbol: "wOETH",
        base_currency: "OETH",
        address: "0x70843CE8E54d2b87Ee02B1911c06EA5632cd07d3",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "ETH",
        address: "0xb4479d436DDa5c1A79bD88D282725615202406E3",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "eETH",
        address: "0x72EC6bF88effEd88290C66DCF1bE2321d80502f5",
    },
    ChainlinkPriceFeed {
        token_symbol: "wstETH",
        base_currency: "stETH",
        address: "0xe59EBa0D492cA53C6f46015EEa00517F2707dc77",
    },
    ChainlinkPriceFeed {
        token_symbol: "ynETH",
        base_currency: "ETH",
        address: "0x4D4f9BF097DfE85C513E29D770e7CaceD9C07801",
    },
];

pub static ZKSYNC_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "USD",
        address: "0x2137c69DCb41f611Cc8f39F8A98047e774d6ED74",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "USD",
        address: "0x4Cba285c15e3B540C474A114a7b135193e4f1EA6",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "USD",
        address: "0x5d336664b5D7A332Cd256Bf68CbF2270C6202fc6",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOGE",
        base_currency: "USD",
        address: "0x2cC24D99500a134ea7f78736b5C329C84599fb1B",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "USD",
        address: "0x6D41d1dc818112880b40e26BD6FD347E41008eDA",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "ETH",
        address: "0xB66325FC0F8aA6dE6FeDFF4e51e54025cEea51eE",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "USD",
        address: "0x1b5a683579f53b9E30B538F70544444389633c75",
    },
    ChainlinkPriceFeed {
        token_symbol: "PEPE",
        base_currency: "USD",
        address: "0x7a6333CaC589e9B11b1fEC190a5828272A2893B5",
    },
    ChainlinkPriceFeed {
        token_symbol: "SOL",
        base_currency: "USD",
        address: "0x498232F0a52D4e94A6e0Ea93D63C07Bc63133009",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "USD",
        address: "0x93A08A9D592101938D4a63c86d0989d7018c00D9",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "USD",
        address: "0x1824D297C6d6D311A204495277B63e943C2D376E",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "USD",
        address: "0xB615075979AE1836B476F651f1eB79f0Cd3956a9",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDe",
        base_currency: "USD",
        address: "0x4899faF0b6c36620168D00e3DbD4CB9361244c4d",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZK",
        base_currency: "USD",
        address: "0xD1ce60dc8AE060DDD17cA8716C96f193bC88DD13",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0x600E54Fc4cF6148Cd6696A3D37F5f672829BF7C2",
    },
    ChainlinkPriceFeed {
        token_symbol: "rswETH",
        base_currency: "ETH",
        address: "0x92c2ed09B096470DF40973231Dac5aC987F9b8F9",
    },
    ChainlinkPriceFeed {
        token_symbol: "sUSDe",
        base_currency: "USDe",
        address: "0x97920183c36B022B46D6C14b9dA36c5f31A98C6A",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "eETH",
        address: "0x8D3184a992f93729b249407C33F1e78abE0d650e",
    },
    ChainlinkPriceFeed {
        token_symbol: "wstETH",
        base_currency: "stETH",
        address: "0x24a0C9404101A8d7497676BE12F10aEa356bAC28",
    },
];

pub static BNB_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
    ChainlinkPriceFeed {
        token_symbol: "1INCH",
        base_currency: "USD",
        address: "0x9a177Bb9f5b6083E962f9e62bD21d4b5660Aeb03",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAPL",
        base_currency: "USD",
        address: "0xb7Ed5bE7977d61E83534230f3256C021e0fae0B6",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE",
        base_currency: "USD",
        address: "0xA8357BF572460fC40f4B0aCacbB2a6A61c89f475",
    },
    ChainlinkPriceFeed {
        token_symbol: "AAVE Network Emergency Count",
        base_currency: "",
        address: "0xcabb46FfB38c93348Df16558DF156e9f68F9F7F1",
    },
    ChainlinkPriceFeed {
        token_symbol: "ADA",
        base_currency: "BNB",
        address: "0x2d5Fc41d1365fFe13d03d91E82e04Ca878D69f4B",
    },
    ChainlinkPriceFeed {
        token_symbol: "ADA",
        base_currency: "USD",
        address: "0xa767f745331D267c7751297D982b050c93985627",
    },
    ChainlinkPriceFeed {
        token_symbol: "ALPACA",
        base_currency: "USD",
        address: "0xe0073b60833249ffd1bb2af809112c2fbf221DF6",
    },
    ChainlinkPriceFeed {
        token_symbol: "AMZN",
        base_currency: "USD",
        address: "0x51d08ca89d3e8c12535BA8AEd33cDf2557ab5b2a",
    },
    ChainlinkPriceFeed {
        token_symbol: "ATOM",
        base_currency: "USD",
        address: "0xb056B7C804297279A9a673289264c17E6Dc6055d",
    },
    ChainlinkPriceFeed {
        token_symbol: "AUD",
        base_currency: "USD",
        address: "0x498F912B09B5dF618c77fcC9E8DA503304Df92bF",
    },
    ChainlinkPriceFeed {
        token_symbol: "AVAX",
        base_currency: "USD",
        address: "0x5974855ce31EE8E1fff2e76591CbF83D7110F151",
    },
    ChainlinkPriceFeed {
        token_symbol: "AXS",
        base_currency: "USD",
        address: "0x7B49524ee5740c99435f52d731dFC94082fE61Ab",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAND",
        base_currency: "BNB",
        address: "0x3334bF7ec892Ca03D1378B51769b7782EAF318C4",
    },
    ChainlinkPriceFeed {
        token_symbol: "BAND",
        base_currency: "USD",
        address: "0xC78b99Ae87fF43535b0C782128DB3cB49c74A4d3",
    },
    ChainlinkPriceFeed {
        token_symbol: "BCH",
        base_currency: "BNB",
        address: "0x2a548935a323Bb7329a5E3F1667B979f16Bc890b",
    },
    ChainlinkPriceFeed {
        token_symbol: "BCH",
        base_currency: "USD",
        address: "0x43d80f616DAf0b0B42a928EeD32147dC59027D41",
    },
    ChainlinkPriceFeed {
        token_symbol: "BETH",
        base_currency: "USD",
        address: "0x2A3796273d47c4eD363b361D3AEFb7F7E2A13782",
    },
    ChainlinkPriceFeed {
        token_symbol: "BNB",
        base_currency: "USD",
        address: "0x0567F2323251f0Aab15c8dFb1967E4e8A7D42aeE",
    },
    ChainlinkPriceFeed {
        token_symbol: "BRL",
        base_currency: "USD",
        address: "0x5cb1Cb3eA5FB46de1CE1D0F3BaDB3212e8d8eF48",
    },
    ChainlinkPriceFeed {
        token_symbol: "BSW",
        base_currency: "USD",
        address: "0x08E70777b982a58D23D05E3D7714f44837c06A21",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "BNB",
        address: "0x116EeB23384451C78ed366D4f67D5AD44eE771A0",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "ETH",
        address: "0xf1769eB4D1943AF02ab1096D7893759F6177D6B8",
    },
    ChainlinkPriceFeed {
        token_symbol: "BTC",
        base_currency: "USD",
        address: "0x264990fbd0A4796A3E3d8E37C4d5F87a3aCa5Ebf",
    },
    ChainlinkPriceFeed {
        token_symbol: "C98",
        base_currency: "USD",
        address: "0x889158E39628C0397DC54B84F6b1cbe0AaEb7FFc",
    },
    ChainlinkPriceFeed {
        token_symbol: "CAKE",
        base_currency: "BNB",
        address: "0xcB23da9EA243f53194CBc2380A6d4d9bC046161f",
    },
    ChainlinkPriceFeed {
        token_symbol: "CAKE",
        base_currency: "USD",
        address: "0xB6064eD41d4f67e353768aA239cA86f4F73665a1",
    },
    ChainlinkPriceFeed {
        token_symbol: "CFX",
        base_currency: "USD",
        address: "0xe3cA2f3Bad1D8327820f648C759f17162b5383ae",
    },
    ChainlinkPriceFeed {
        token_symbol: "CHF",
        base_currency: "USD",
        address: "0x964261740356cB4aaD0C3D2003Ce808A4176a46d",
    },
    ChainlinkPriceFeed {
        token_symbol: "CHR",
        base_currency: "USD",
        address: "0x1f771B2b1F3c3Db6C7A1d5F38961a49CEcD116dA",
    },
    ChainlinkPriceFeed {
        token_symbol: "COIN",
        base_currency: "USD",
        address: "0x2d1AB79D059e21aE519d88F978cAF39d74E31AEB",
    },
    ChainlinkPriceFeed {
        token_symbol: "COMP",
        base_currency: "USD",
        address: "0x0Db8945f9aEf5651fa5bd52314C5aAe78DfDe540",
    },
    ChainlinkPriceFeed {
        token_symbol: "CRV",
        base_currency: "USD",
        address: "0x2e1C3b6Fcae47b20Dd343D9354F7B1140a1E6B27",
    },
    ChainlinkPriceFeed {
        token_symbol: "Calculated BNBx",
        base_currency: "USD",
        address: "0xc4429B539397a3166eF3ef132c29e34715a3ABb4",
    },
    ChainlinkPriceFeed {
        token_symbol: "Calculated SAVAX",
        base_currency: "USD",
        address: "0x3b37C6f1e3207DE5a4664E837072Bd9A25269B39",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "BNB",
        address: "0x8EC213E7191488C7873cEC6daC8e97cdbAdb7B35",
    },
    ChainlinkPriceFeed {
        token_symbol: "DAI",
        base_currency: "USD",
        address: "0x132d3C0B1D2cEa0BC552588063bdBb210FDeecfA",
    },
    ChainlinkPriceFeed {
        token_symbol: "DODO",
        base_currency: "USD",
        address: "0x87701B15C08687341c2a847ca44eCfBc8d7873E1",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOGE",
        base_currency: "USD",
        address: "0x3AB0A0d137D4F946fBB19eecc6e92E64660231C8",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOT",
        base_currency: "BNB",
        address: "0xBA8683E9c3B1455bE6e18E7768e8cAD95Eb5eD49",
    },
    ChainlinkPriceFeed {
        token_symbol: "DOT",
        base_currency: "USD",
        address: "0xC333eb0086309a16aa7c8308DfD32c8BBA0a2592",
    },
    ChainlinkPriceFeed {
        token_symbol: "EOS",
        base_currency: "BNB",
        address: "0xed93F3764334788f2f6628b30e505fe1fc5d1D7b",
    },
    ChainlinkPriceFeed {
        token_symbol: "EOS",
        base_currency: "USD",
        address: "0xd5508c8Ffdb8F15cE336e629fD4ca9AdB48f50F0",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "BNB",
        address: "0x63D407F32Aa72E63C7209ce1c2F5dA40b3AaE726",
    },
    ChainlinkPriceFeed {
        token_symbol: "ETH",
        base_currency: "USD",
        address: "0x9ef1B8c0E4F7dc8bF5719Ea496883DC6401d5b2e",
    },
    ChainlinkPriceFeed {
        token_symbol: "EUR",
        base_currency: "USD",
        address: "0x0bf79F617988C472DcA68ff41eFe1338955b9A80",
    },
    ChainlinkPriceFeed {
        token_symbol: "FDUSD",
        base_currency: "USD",
        address: "0x390180e80058A8499930F0c13963AD3E0d86Bfc9",
    },
    ChainlinkPriceFeed {
        token_symbol: "FIL",
        base_currency: "USD",
        address: "0xE5dbFD9003bFf9dF5feB2f4F445Ca00fb121fb83",
    },
    ChainlinkPriceFeed {
        token_symbol: "FRAX",
        base_currency: "USD",
        address: "0x13A9c98b07F098c5319f4FF786eB16E22DC738e1",
    },
    ChainlinkPriceFeed {
        token_symbol: "FTM",
        base_currency: "USD",
        address: "0xe2A47e87C0f4134c8D06A41975F6860468b2F925",
    },
    ChainlinkPriceFeed {
        token_symbol: "FXS",
        base_currency: "USD",
        address: "0x0E9D55932893Fb1308882C7857285B2B0bcc4f4a",
    },
    ChainlinkPriceFeed {
        token_symbol: "GBP",
        base_currency: "USD",
        address: "0x8FAf16F710003E538189334541F5D4a391Da46a0",
    },
    ChainlinkPriceFeed {
        token_symbol: "GME",
        base_currency: "USD",
        address: "0x66cD2975d02f5F5cdEF2E05cBca12549B1a5022D",
    },
    ChainlinkPriceFeed {
        token_symbol: "GMT",
        base_currency: "USD",
        address: "0x8b0D36ae4CF8e277773A7ba5F35c09Edb144241b",
    },
    ChainlinkPriceFeed {
        token_symbol: "GOOGL",
        base_currency: "USD",
        address: "0xeDA73F8acb669274B15A977Cb0cdA57a84F18c2a",
    },
    ChainlinkPriceFeed {
        token_symbol: "HIGH",
        base_currency: "USD",
        address: "0xdF4Dd957a84F798acFADd448badd2D8b9bC99047",
    },
    ChainlinkPriceFeed {
        token_symbol: "ICP",
        base_currency: "USD",
        address: "0x84210d9013A30C6ab169e28840A6CC54B60fa042",
    },
    ChainlinkPriceFeed {
        token_symbol: "INJ",
        base_currency: "USD",
        address: "0x63A9133cd7c611d6049761038C16f238FddA71d7",
    },
    ChainlinkPriceFeed {
        token_symbol: "INR",
        base_currency: "USD",
        address: "0xeF0a3109ce97e0B58557F0e3Ba95eA16Bfa4A89d",
    },
    ChainlinkPriceFeed {
        token_symbol: "JPM",
        base_currency: "USD",
        address: "0x8f26ba94180371baA2D2C143f96b6886DCACA250",
    },
    ChainlinkPriceFeed {
        token_symbol: "JPY",
        base_currency: "USD",
        address: "0x22Db8397a6E77E41471dE256a7803829fDC8bC57",
    },
    ChainlinkPriceFeed {
        token_symbol: "KAVA",
        base_currency: "USD",
        address: "0x12bf0C3f7D5aca9E711930d704dA2774358d9210",
    },
    ChainlinkPriceFeed {
        token_symbol: "KLAY",
        base_currency: "USD",
        address: "0xfD07b211044572898cDbcb1435f0a1369Fd15726",
    },
    ChainlinkPriceFeed {
        token_symbol: "KNC",
        base_currency: "USD",
        address: "0xF2f8273F6b9Fc22C90891DC802cAf60eeF805cDF",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINA",
        base_currency: "USD",
        address: "0x38393201952f2764E04B290af9df427217D56B41",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "BNB",
        address: "0xB38722F6A608646a538E882Ee9972D15c86Fc597",
    },
    ChainlinkPriceFeed {
        token_symbol: "LINK",
        base_currency: "USD",
        address: "0xca236E327F629f9Fc2c30A4E95775EbF0B89fac8",
    },
    ChainlinkPriceFeed {
        token_symbol: "LIT",
        base_currency: "USD",
        address: "0x83766bA8d964fEAeD3819b145a69c947Df9Cb035",
    },
    ChainlinkPriceFeed {
        token_symbol: "LTC",
        base_currency: "BNB",
        address: "0x4e5a43A79f53c0a8e83489648Ea7e429278f8b2D",
    },
    ChainlinkPriceFeed {
        token_symbol: "LTC",
        base_currency: "USD",
        address: "0x74E72F37A8c415c8f1a98Ed42E78Ff997435791D",
    },
    ChainlinkPriceFeed {
        token_symbol: "MASK",
        base_currency: "USD",
        address: "0x4978c0abE6899178c1A74838Ee0062280888E2Cf",
    },
    ChainlinkPriceFeed {
        token_symbol: "MATIC",
        base_currency: "USD",
        address: "0x7CA57b0cA6367191c94C8914d7Df09A57655905f",
    },
    ChainlinkPriceFeed {
        token_symbol: "META",
        base_currency: "USD",
        address: "0xfc76E9445952A3C31369dFd26edfdfb9713DF5Bb",
    },
    ChainlinkPriceFeed {
        token_symbol: "MRNA",
        base_currency: "USD",
        address: "0x6101F4DFBb24Cac3D64e28A815255B428b93639f",
    },
    ChainlinkPriceFeed {
        token_symbol: "MS",
        base_currency: "USD",
        address: "0x6b25F7f189c3f26d3caC43b754578b67Fc8d952A",
    },
    ChainlinkPriceFeed {
        token_symbol: "MSFT",
        base_currency: "USD",
        address: "0x5D209cE1fBABeAA8E6f9De4514A74FFB4b34560F",
    },
    ChainlinkPriceFeed {
        token_symbol: "MXN",
        base_currency: "USD",
        address: "0x16c0C1f971b1780F952572670A9d5ce4123582a1",
    },
    ChainlinkPriceFeed {
        token_symbol: "NEAR",
        base_currency: "USD",
        address: "0x0Fe4D87883005fCAFaF56B81d09473D9A29dCDC3",
    },
    ChainlinkPriceFeed {
        token_symbol: "NFLX",
        base_currency: "USD",
        address: "0x1fE6c9Bd9B29e5810c2819f37dDa8559739ebeC9",
    },
    ChainlinkPriceFeed {
        token_symbol: "NULS",
        base_currency: "USD",
        address: "0xaCFBE73231d312AC6954496b3f786E892bF0f7e5",
    },
    ChainlinkPriceFeed {
        token_symbol: "NVDA",
        base_currency: "USD",
        address: "0xea5c2Cbb5cD57daC24E26180b19a929F3E9699B8",
    },
    ChainlinkPriceFeed {
        token_symbol: "ONG",
        base_currency: "USD",
        address: "0xcF95796f3016801A1dA5C518Fc7A59C51dcEf793",
    },
    ChainlinkPriceFeed {
        token_symbol: "ONT",
        base_currency: "USD",
        address: "0x887f177CBED2cf555a64e7bF125E1825EB69dB82",
    },
    ChainlinkPriceFeed {
        token_symbol: "PAXG",
        base_currency: "USD",
        address: "0x7F8caD4690A38aC28BDA3D132eF83DB1C17557Df",
    },
    ChainlinkPriceFeed {
        token_symbol: "PFE",
        base_currency: "USD",
        address: "0xe96fFdE2ba50E0e869520475ee1bC73cA2dEE326",
    },
    ChainlinkPriceFeed {
        token_symbol: "PHP",
        base_currency: "USD",
        address: "0x1CcaD765D39Aa2060eB4f6dD94e5874db786C16f",
    },
    ChainlinkPriceFeed {
        token_symbol: "QQQ",
        base_currency: "USD",
        address: "0x9A41B56b2c24683E2f23BdE15c14BC7c4a58c3c4",
    },
    ChainlinkPriceFeed {
        token_symbol: "RDNT",
        base_currency: "USD",
        address: "0x20123C6ebd45c6496102BeEA86e1a6616Ca547c6",
    },
    ChainlinkPriceFeed {
        token_symbol: "REEF",
        base_currency: "USD",
        address: "0x46f13472A4d4FeC9E07E8A00eE52f4Fa77810736",
    },
    ChainlinkPriceFeed {
        token_symbol: "SGD",
        base_currency: "USD",
        address: "0x3065b2369820f76C829b9BBCAF4B90F9f47d6314",
    },
    ChainlinkPriceFeed {
        token_symbol: "SHIB",
        base_currency: "USD",
        address: "0xA615Be6cb0f3F36A641858dB6F30B9242d0ABeD8",
    },
    ChainlinkPriceFeed {
        token_symbol: "SOL",
        base_currency: "USD",
        address: "0x0E8a53DD9c13589df6382F13dA6B3Ec8F919B323",
    },
    ChainlinkPriceFeed {
        token_symbol: "SPELL",
        base_currency: "USD",
        address: "0x47e01580C537Cd47dA339eA3a4aFb5998CCf037C",
    },
    ChainlinkPriceFeed {
        token_symbol: "SPY",
        base_currency: "USD",
        address: "0xb24D1DeE5F9a3f761D286B56d2bC44CE1D02DF7e",
    },
    ChainlinkPriceFeed {
        token_symbol: "STONE",
        base_currency: "ETH",
        address: "0xC6A1314E89d01517a90AE4b0d9d5e499A324B283",
    },
    ChainlinkPriceFeed {
        token_symbol: "SUSHI",
        base_currency: "USD",
        address: "0xa679C72a97B654CFfF58aB704de3BA15Cde89B07",
    },
    ChainlinkPriceFeed {
        token_symbol: "SXP",
        base_currency: "USD",
        address: "0xE188A9875af525d25334d75F3327863B2b8cd0F1",
    },
    ChainlinkPriceFeed {
        token_symbol: "TRX",
        base_currency: "USD",
        address: "0xF4C5e535756D11994fCBB12Ba8adD0192D9b88be",
    },
    ChainlinkPriceFeed {
        token_symbol: "TSLA",
        base_currency: "USD",
        address: "0xEEA2ae9c074E87596A85ABE698B2Afebc9B57893",
    },
    ChainlinkPriceFeed {
        token_symbol: "TUSD",
        base_currency: "USD",
        address: "0xa3334A9762090E827413A7495AfeCE76F41dFc06",
    },
    ChainlinkPriceFeed {
        token_symbol: "TWT",
        base_currency: "BNB",
        address: "0x7E728dFA6bCa9023d9aBeE759fDF56BEAb8aC7aD",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "BNB",
        address: "0x25298F020c3CA1392da76Eb7Ac844813b218ccf7",
    },
    ChainlinkPriceFeed {
        token_symbol: "UNI",
        base_currency: "USD",
        address: "0xb57f259E7C24e56a1dA00F66b55A5640d9f9E7e4",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "BNB",
        address: "0x45f86CA2A8BC9EBD757225B19a1A0D7051bE46Db",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDC",
        base_currency: "USD",
        address: "0x51597f405303C4377E36123cBc172b13269EA163",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "BNB",
        address: "0xD5c40f5144848Bd4EF08a9605d860e727b991513",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDT",
        base_currency: "USD",
        address: "0xB97Ad0E74fa7d920791E90258A6E2085088b4320",
    },
    ChainlinkPriceFeed {
        token_symbol: "USDV",
        base_currency: "USD",
        address: "0x75e9262c8E87af54fAdF2f7A1BdD975D1C7A5479",
    },
    ChainlinkPriceFeed {
        token_symbol: "VET",
        base_currency: "USD",
        address: "0x9f1fD2cEf7b226D555A747DA0411F93c5fe74e13",
    },
    ChainlinkPriceFeed {
        token_symbol: "WIN",
        base_currency: "USD",
        address: "0x9e7377E194E41d63795907c92c3EB351a2eb0233",
    },
    ChainlinkPriceFeed {
        token_symbol: "WING",
        base_currency: "USD",
        address: "0xf7E7c0ffCB11dAC6eCA1434C67faB9aE000e10a7",
    },
    ChainlinkPriceFeed {
        token_symbol: "WOO",
        base_currency: "USD",
        address: "0x02Bfe714e78E2Ad1bb1C2beE93eC8dc5423B66d4",
    },
    ChainlinkPriceFeed {
        token_symbol: "WTI",
        base_currency: "USD",
        address: "0xb1BED6C1fC1adE2A975F54F24851c7F410e27718",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAG",
        base_currency: "USD",
        address: "0x817326922c909b16944817c207562B25C4dF16aD",
    },
    ChainlinkPriceFeed {
        token_symbol: "XAU",
        base_currency: "USD",
        address: "0x86896fEB19D8A607c3b11f2aF50A0f239Bd71CD0",
    },
    ChainlinkPriceFeed {
        token_symbol: "XLM",
        base_currency: "USD",
        address: "0x27Cc356A5891A3Fe6f84D0457dE4d108C6078888",
    },
    ChainlinkPriceFeed {
        token_symbol: "XRP",
        base_currency: "BNB",
        address: "0x798A65D349B2B5E6645695912880b54dfFd79074",
    },
    ChainlinkPriceFeed {
        token_symbol: "XRP",
        base_currency: "USD",
        address: "0x93A67D414896A280bF8FFB3b389fE3686E014fda",
    },
    ChainlinkPriceFeed {
        token_symbol: "XTZ / ",
        base_currency: "BNB",
        address: "0x8264d6983B219be65C4D62f1c82B3A2999944cF2",
    },
    ChainlinkPriceFeed {
        token_symbol: "XTZ",
        base_currency: "USD",
        address: "0x9A18137ADCF7b05f033ad26968Ed5a9cf0Bf8E6b",
    },
    ChainlinkPriceFeed {
        token_symbol: "XVS",
        base_currency: "BNB",
        address: "0xf369A13E7f2449E58DdE8302F008eE9131f8d859",
    },
    ChainlinkPriceFeed {
        token_symbol: "XVS",
        base_currency: "USD",
        address: "0xBF63F430A79D4036A5900C19818aFf1fa710f206",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "BNB",
        address: "0xF841761481DF19831cCC851A54D8350aE6022583",
    },
    ChainlinkPriceFeed {
        token_symbol: "YFI",
        base_currency: "USD",
        address: "0xD7eAa5Bf3013A96e3d515c055Dbd98DbdC8c620D",
    },
    ChainlinkPriceFeed {
        token_symbol: "ZAR",
        base_currency: "USD",
        address: "0xDE1952A1bF53f8E558cc761ad2564884E55B2c6F",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0xD164a1559e78B9e6eacb12B2E627F31d2f603C0f",
    },
    ChainlinkPriceFeed {
        token_symbol: "ezETH",
        base_currency: "ETH",
        address: "0x06F34EdD61Be3b2Ff3F630B500dF81eeA8312350",
    },
    ChainlinkPriceFeed {
        token_symbol: "weETH",
        base_currency: "eETH",
        address: "0xF37Be32598E9851f785acA86c2162e7C1A8466dd",
    },
];

pub static FANTOM_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
  // ... ADD HERE
];

pub static METIS_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
  // ... ADD HERE
];

pub static SCROLL_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
  // ... ADD HERE
];

pub static AVALANCHE_PRICE_FEEDS: &[ChainlinkPriceFeed] = &[
  // ... ADD HERE
];
