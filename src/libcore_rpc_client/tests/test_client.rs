use std::{fmt::Write as FmtWrite, fs::File, io::prelude::*};
use veil_core_rpc::{
    client::{Client, ClientConfig},
    Request, Result,
};

// TODO: Txs to and from all 3 types
// TODO: Blocks with examples of to and from all types
const BASECOIN_TX_HASH: &str = "89727907a87458249ef6719987b9d29fb0a091141c2bf7b7c5c351f731429ea9";
const RINGCT_TX_HASH: &str = "da6ebf52aacbb3247f442525baac97323dda87b127f838d83c823d62b51ea557";
const BLOCK_HEIGHT: u64 = 457_623;
const BLOCK_HASH: &str = "8e874811cef61fba5f4a34be5d2ae6ab592dfb06bc3cbcb37affabc21478e67b";
const BASECOIN_BLOCK_HASH: &str =
    "d518e95ea16acf20a8082a91242d8e1c43d3a947b6119072fb89949d8309f026";
const RINGCT_TX_RAW: &str = "0200010000000001050000000b000000000000000000000000000000000000000000000000000000a0ffffff00ffffffff01a5020b2bc3bce9478f80c45b08b48019ea104e2b24d53aa15b8ffa6c0d7b45250c27037dbf0ab3d251a40286a764ae2eaf5e5d9474283f43eeff2354e5290abb72fdf4028e681c60e3c5d2bdc265664f926c5680d5d7c054e8046d5c0dd1204e9582daed029f290e7b44bb30ecfec4f562d8618841341abf3c06ffd7dc8ac1b8eec5b9f700024859d9ddb5a761f05fa979e23543c88f28c6819e424c21d91e7b80d11b85a5a203040306c53d0303158db8e134c082b584ccd6f9b839ae9e72c84e1474efd180f1e2cc59c4bfc4890954c3688e4858f4e898bd7be9fedf38902a98a12d6a7815b07d35dc3649b5feb2210292d19d2137ce9145bdf59c6adf7f3b5abb118e37cef65307b5c8d1c6d80b9ca1fd040a401f2027f9c59d05227f7b3d1153a1e43224c636cc1befff92cbff26b9dae6c2a886401b9ab57bb47afc561b6fa5a0d685e6bfe1d4deff613dbb1cd8c801c2a8e7b3fa67d07936c4d7f45b9cecf7eae73a00e391a372be5d8b27368b6f16d4c41a7bc596bf4949ac38da0e55bcfebf7c2e8064d06b5f72a0cbb8c3f3e7f235f26153da22882bf8931549862cafb311b87365e8f99b689058f46d73102650dd7e9c11db03590e9e1f27f3729bac21dd42c00af3459c0a1a109bb975f6224f17fcd1fb48e351c68e3cc5269a668cb06f3b0e4f8eaa5634cfcdf816e024492b7eae0c45a92a6bb27b01ba5d4997f9c6c3a6576fa57b999c824016332b3021c14dfa194e96b2c72ed276dfdf85876319d1228e9b87b102203cc595ce9988a4885a8d815e6162dcd047db82d59da350eb1b5f164cb384200bbe8a66621af0a735ee42dd3380eff8db6e0ee8b4ce1fa7659a35e2cbd49ccb0bb16ec9d62ab94cadb2eff648fccbfb1467a8fec40038c176941d7b3d87763e8c2b5ba7e57ff437e74162a737fdf8bdce008819537ce834667c5042b5a27b44a539e1e682f83fc05ea39b1ca8bda3c213e5dd3daad8dfe2c024f36e221c96eebd24a7b8bb9f906a5c008e060ea83dceb86fe4269fdf10fb8d3735166c433da990503d52855ae44a9935a0479705685120fc1d992bbe8757a5c162c0f0b06bcee7ea9d4a556bb8e78305c23fbf77cb10776ecf73c2a1b7cc47e591c0b558ca014ba4cd65c3b17738cc502a635c705ccf95e1547164ba3fc0f06705fadb63a8d99dd6b35776553ddb72f3a6a84fa76d33e72292d7f6e46a6ed63e06fd2857abf9e7192f66fe270be0979902984ad0749d7fdce099c0b7533ca362d4b5ec2a6324dd1f7b27e0f5ddd2a94dc06dd02f5f3cb8ffeea0e6de313dfbb7b70151176c10aa4ea55a1ccdd51d3971b141ba5e7b765b9dda7501f0de99419fbfef6296eaf9257575b2a8ed9af74ab14ab46c5e6eb640bab0ed16703fadbe240a1f2c78659346262d0abfba5ff79cefb4298012f1aa6496e3e6795d19e2ad78312ac5d63ec6f2344b0d1ea1a1e5bc97794570920597119696be74b718cd95ff690e559ef08f09b4989fd4436caa090ec60cf5865a6075bc458ee12222021c8aa36e86bf6ac3ab8b5ce46d1d4db15295a15b69ecb969d6d5a2dd84b5427cf8fc21c9ba6a881dbb8b576aa0fa1dd50e2a8e247e55e701966835950e3845a3b245297cb0f6f688e24b95125b4e153f638a2a4e96033d3ad444ccfaca9e01fc4571e0b2837cced6f7525e3ac709bdce3f4d0f9e179b5581f1e196ac45d1ea7b108897d4fe949740b6b58b88225b7bd143437c2fae951e1356c852b2b3cc8417a7aa41c97c31dc1d5dc1592e3711be86c87ec29dddcade5d74d9587668e5ff0cbc0083f9ae7b01c86f5601838fa3b2ec773c50acd816356221152bc84b2d942d4e3b0fca2e090138e61818357d75dc7600554f6c4931fdd537ee21476cbabf5b4ef7171b85c1dd5d6cf70359593f502ce787f3e1473b8bf2aeddba4b4e6c17a6811dd28fd8eab99c586beabd9f7b243c760f372be9cd27923aa6a273b20975e8cbfefe8af5305ddf3af8b676204361be6b2df107d3a4a4dea159833e47b6304e301ced4193d25e014945061dcffd906bf614821a41197d1fd855e6505c82bf31ee51197bb4e6a29d65942af8cb680b32aa0122dd2841c4a5b0fc8a4ab29e4b2298e13567091503b58f89f9668a9089266a33fe71dd138d30ed5769a1ca65cf775a908bb81126f6e8f8b908eef18d416d9ee78d62a90f2e16ead486845b0997daf5404a5f13e97f054c3fc29572be1229c36ce0367c12ca1e12b40a58be42f8f18fac96e20e465dcc7769f028fedc839df118e137db8c8778d1b94f22fc57737bafef0052cce55fe279130f08e17bd2a56c9fc6d8408becceabddd30aded23496f83335c1375f516049bc048c8e5f93a73b585b40c1b0850b06f142b70259676494189834946836ad029801251098ee6f708ce1508557424283a2e999a8f3ca2bb9000fcbe5b6f3c83567964220c0a4ee3ac92c61996261d50b4abbca50a822f59ebbabab26ee1fda6217a4d1e0c3f9bc9cca8d1283edd858881fef43ef6664d81093752f01247595b2018070f5ac8a3d3c1d8f51c3bceab25e6d190e782ebef15ec52167c4362a9efa117301ca86520e33828882d7265cff895dec7c7487bdf6eb73d771b7d3c8909b80a76f8602ddfd187567379dd02086541881dccb1c70691a6679a0283531ee1eb070c6ca56710eb5d0aceb71efc4c926072ee435f918eeca2e226211e6199b47f80571ee9a00159110e85edb194cbd5099ebef6c12f7217212234c25b21e6b353409b0eac736c6507e2343dd9dd1af5912b910a6332e334c5eb9587c8f708b16431995418012f02eda7201f0d3278a69a4216dc56ea2a9df2148a8a85595ef75292e03f29655febda1d4c0e41042b37d10b5a51f8cbc89818570cea920552b0e6901165c245850e4183890ef14e0ad8dfe7c619bce425511bb7a5b526f7c67f90a460b0b8f7b197705f58f470b716b25d71c8be0d200ef304ce5f8fa2c96b0988b4230d78056f11a94f27bd539cff786e1e364ecca18280baed575221ef1fc8a070c76ee3a48d4433a904516fada62bbb62156e91cc6ca69299488d7c2ce7d9e2be9109107173d28cca5ad2f55418eeb1d2da5e8b3994c1c3e55d0cbf3f6c7b651506c862c4a1cd0694436a052d6ba9ecdc6b4ccfe5ac666e34bee1de82aa375966e32e2fd964679d402168589729476e687d4b2bc34bb77467c23d09f39be92497974cfe26f524a720bcb4592c0da8304f10c78edce82616b9fd726e8085f65957f20dfe50b30c6b6edf2dd9f8b737b35b7f789b9ac66e70d813d67c2a657954f1c36343153e2e5499b9581b5834970863897e5f72dc1adedd9fe10215a7d758e595cbca5560222399dc60d50e21ece5475ff15461fb5cd5f904eb6315b62fc821de02ac9f2fe95b80ce818a4f4fcdd78935053a1fc2494d00f0c0bd41fa3448e6a3abe2e2ac5368cd8ff928cdc8c916dfa6a572d67c6202c824d189196fa3bbd339da0e6ef3f56101834773c7376e3a2ba8a3fc62f5d5f42de4355c1ab8d288f14813a15f4f2c33e1c1df0d054d137ff9d682910893876986bbb67edd8398ed7a32a59059e2dfc2b522b76c37ae829ca970b0863b181a73f673801725c6037a2cee2ea1e0b80eeb5dff8fd81e67bfbd54ecc87d7c4710de8a1cd611417805bf7871345ab6e7fa7cbba0a8b6558178dbebd3b022d41d4218c479dc7f23f32697512044e8d01c1ab120a39e9115c67856739108b7ab0c07126a38bc3d76bac8bbd5dd4f309f18dd0f4252eed33a7399238b72f55548e71a87a7fea0564ff40c0d8a3b1ea6ccce64a84e0132a7aac82af9ef973838b63d9932c578887a35db0d7e1bc8f49e784c8e8eaeedf65fb96ed3e7deb6e93531c6b60337dd340cb3e88475ee622e2963bc15fc712a4ef44f4589cd622c66d2bed0e8160a227a470570831d27938c7ce128882231bf19773e476688bada8c3b7d7ed799bb8575ff621d1f84f3373352e87085587424a0f5ba803035ad4ccd8421b452986d5f8e6e15ba39deb230f99cba6546ccc4f6c29f526b59609c91a15ad093c8564ce74fb1d358553c3478d63fcfba25e27ef4dc80825bf95322102d6b17afa355318a916919ca2e20b611457949d6137333f2c7664b37189402cb8fd040a431fd81cff0325f476678d3a11d57cfa1e6c7a40a9587f7f8040ccc6bea2270dd22f4b1d39b130a2093a8a6f9285d75c5d75c112a6ced0d7b7a65ea87b1b0d9c8b0d3fe27148bc6b99bdd2efc897dcd3363eb05825fbdd24470d2f611fd19ba382396c7823f88735d65666c49bbd4fe1d97211a1b16b55b1ef4bda913c3d1d843e63b7d73bbf13a5a22cd4ea97a63ba3ed3ca645eec47fcad5f1cbae9649fdfa51e889e94bb666a642106acca0173a8978b91f00b717fb299843bae3b62ff207daa9d6e49e749b1bbf2557dc1a70dd1423576544a26d78ed8fd68e49b82b15b2b95ccc1dbff7ac0fd12227e6dddb9fce73a24228103c856be6cc2bbebdc6ab954969997f4ed18d87463f967e8e6bd88076c2aa7fe8f69b5ed05f31b1dc67aac50f5da269dca7658b52a2115da3455033701de836ca08e0b2d7188f65689f4bfe8a3880a225bc33e14f33e1dfe32adb2dfddf4b4476f2275fae4a70bf30bdadabb82f6bfb0f6c8f0635599e2b4e5ea7f705c9f68ea91d3424e3d9b22f0b8acb1374eedaddc36124658feb07ade6152f7058515491d72c3a9d8e28bbb522f1f6cb72c4f6d6b973cfa94d22ba35c2d4ecff622eca4213bcad1c1a3736ff695836594412692b69dcb1b06ddc8a99bc253da696ff6bca9d1fc17c99e7b23df3bd0ac7d92b6bfd78e3b9bd177c8ce6a48b7ab44f490f07214aa2ba6b4ce17ee34340bc2130f675f79e1a37959935c5fbb1f3d1fef1ad1b23e0c81fbee3c565621b8764279e5a422015531cea301f5d64db2ba3adc5c38e332cc4fc81442f294dee1ac0d7e7f0b82958500cde8f3e776c960c775d1a5575cec5d588af22e47e2d664acd9d4511aaa352d752ee7f3b4f99322e509e3701f497a1102b57e1080f03d189fe96f6597642ce10507134f1f22b14e4956bc81ecf460a9a8fea5550482867028fb49a26bbedadc1d0cf307f073ec9282a617ef68acf0b8f346b955542492be92603f9486e9e2e43472cfc10fadafff5ea3d9d651590f067148e9fa549e59fba161ea3b6d4638011ef19dbc6b1ff51c377ab767cd749072bf306d236eceae3bce933219d890a51917487e125b69b94f83ff996edf8fc83c803f0bca990343a5b72d37e9524616d51b9abf8cffb0bb1cd5401a50bac0e944e71063775de860985c2e529349c154e60ab4f314b98fdb6151cc58382d0a500617ba642aafadb6c2deca7a82663c90fca0c639d0dbe5c73af3b74fc71926ee4acbf2764151b58ca7d2ce1ee26471688dfda64626e4ecf730946b7702f7bcfdfb10271df2a2183c143558d87ab3c28b537587959d977f38433859cf06eb7b143cd8b8b91fdea1e07d0d78793b9f00bda10c848037ebb9cc64d07b1f3b14806b08cf369ee63ce327322b6876bdffbad2a37e0e922215b96d6792b9e3dc0dd181d15c324e13278df21334121d73464dacb5a3804f6d1a9edb4c72cb30193ffd825cb1ba88e1e6b128f761c17ed3bce8c67f3a5a301ec5c252d701cde10de4ac8fed82703360d9237d4aa654f1f83ced47c76552e97144a3784bd1fe90b0ad8f3245f75239196eff3850c9aba0b1feaf9dc97471d5902e5acfa698486cedf87fcfd6d846481a87ba9466f27fca604bf37c4d7571d4e6d085e7d026137b7cd96c898b731bc0af1b7956aaa8f01196ad7a29475f7b844e502cf351bc3d3c4a733172216a1a59580a7228c88c841d8f79fcd4f6639e2e12b154b0cfb11fa95100f8bcf2197fadc92ce71875c69d41321d6342ddafe7122ebc35fdaab8bd40d44be1caa19e7c9769b2441dc7fb1b6dc10294553c64431c28df2e7a726eb335e37c9467d600c8b9dafb1a6faa4bca29eaac544392d91c2698bc02b35baade3f6d6cea4c11bd9860a196d1d6622cc0a0906a78c07d69b83739956e5a3faa44826f44db6819846f1acc488237df12b40c7777a0d7feb3a689696ca27bbeb8c424428b4a471e0bba51e926d61b3917f289363b4dc6de81c1daf1f0dfc66cb504b48c9592671819c8e6dc085862984c2f0096d9eaea07ce7238296a6eaa752e1c07b2e2c3bf1510d21328ed891a04f4e7ca30a31c2c5faf0e6e060905df12fbd5e2042d1aad9a6914bb246738c80f7cf969cff4c3d96d68279d6bbb49ab0bd8eaa5b73638ee1daa7308e71c8bccc0263c15ed7621823564f455b4ef147aa61da8d27ed111e82bd99834902ca1c72e71605c15a85f4830ac4b362a45e23020f30f582ae1aab0265af69d4ee8e77f695687a26947dd04cb417f0769a99c321acd2a5494ecf8d172df258417ea8cbd205f5a68c7a9c552710d8333b5e6228e94a346dff4ce2965873e740228ad1af407a825c8323037dafad8f76ba1f3b31bb7838452d15b5eefd47806b20675a62bae7f95351842ee0afee02a33c83b3beccefcc4e2decbb756efe178fbdb60c47e5e7ad03120e41c3315168bc11d8dbed19f178035a9d7c084b0b5f4788bccc9aa1e25c54641a060b2a4d9a3d84e3495c05bf32327c31e6f922be90b743b638f8a0810ef22316c2ba28d44158bebbd9991bd2504d9b5d10f2c12798ac02771fe75d93d4fadce8d2563944b3413ccf9ace1ee59b44365d4488b6b350cfc288fa1eed394367358690bd977f9c175a4201edd8acfb79140b5364ca5e402f2ace8631a349c91f5b3c3fb2eaa34328d4e22d589471031636c79f4dfde89ef6d330efa91ec10e9a20fc26d88208a4f728b220ca3fd5e8697305722b89e2e2bd1f0a15a1904712d55b917b727f11fd0f9deb193667f6c7eddb9e8763635d7c70c0cf5085677953d279c0a2f3529b9e63729371912b1e5d4b396b3a7d21c1dbeb5977baf7d4558909394c8b4d6660b1cdc48c9713e453b3d26c98054e1b54cc2cd59046765bae0a3b310cd421ce8bc087392a8c0dfac1e95781b4e154628ba578cb6c03a4170b585a4165e7613c13b91fee64ae77a66e470b9c7e515d4eef1efe6450601b145488044d12e86cac5e0c90620a947224874f3b90ca7e1a530ad118a3dea36304ac6a2f584bae8486c458fc46fc22bb1fd8673207b6b34f303ff502a105c118ffe28056090839b6c9b76f0435c7f2bade4aeca394ecea0cff8475b9aebb29bb4cf613329cd697ee348b94808557222ceff059d53766cf003a2cb52b83ea5077abe2a0abeba5568c08484734c443a1aca6d3f356a0a07a47bd7986f954c634a7db04e690d82d0f3c59c18942f2a9df528b17f11ecb85410d889261108373f25e458c52e2d6b608291bc9af39519e28305bbad41e5341714e870f5a910ba82ff5fbc095a5abe19c7d35b965d1f56ce6d5b05fb92f81acaeac4d9687b39f028941786406c0251251ecaa928841572438470c4991589de4001018f47beb5b781bab0a3704da120291e1b9a3a37e7ce501cd71a2f0e7cb6ccbc7d1f6a0ed6c4d3a4a2cd6957a404a5f97c776d29d0231476ca9cdff88eb9020efe7631bbe194bb7d326c56a4b489ff60d4e3d6a2fd7cee20034db17445dfdc5cb84679a8fbcf86d3d893a7b625f08390df34dbb48c42175b2974a96d8068eb1245406e2470209f274fb1138e29147b015c091c744d25546bee851d7a950bf5c67e27f5811e1138a655287ba3158272a02a4bf9204ff689a9504d3d201849b04e99c04f0f303839f04999204da9304db9b048f9504dd8204f3a901d69004bf9604b08604eaa104dca104d89204acf102f3a104c99704dea903c79f04e695049aa903a09a0496ae038e9404d3cb03d3a60382a104fa9a04fd9003989b04c9e702c59204e8980494f903cf9c038ccc03a490049c9304949c04f19603be8903c58704f89a04e89604dea104dc9404b4d403de9604aba104fd60080ab0dcd6bed386ce73b17ea89627a5a02b0912a8fe063c02c68557c1f96ebcf0854e5b70dd18b909ebdf644f483ad94c52584ec13f9c3c3d2951840eebde0ccfc88199117bbb830b8c17da8570e7b48135323f40418facaf3befedb8d8970a3a03e3b8be34b0d643b1556d8bf17766433c5f83277066b3aada512a21f1de4b5942111786d0788340f7077410185d8e1cdc04bab409594c8fc778c44fa9b36426872f33f39e6f8c76c62ce978cef93fe9a540437079bd0752f50b9b914c3091bf89f01e9602af691014a8b4c7211ccb90846fd20ccdd94a495b671e29ed2875fc7a85cb2df1c985c298ea54db6f99dc3dce9a6f89dc6d9ff6fc0eecb5f096cd3f8c9b7620f20e1bb9e262709744680036aa36e272f49728612c7b5a18037cb21cbada51097958fe3d47090b27a49faf85aff9af16da202d8bc1c2995310b1790a8fdd66018a3f1932bdaf353bd9b7201a6f35992c4785c55d3b553469956f88dd0dacc7e1a6050c0eef99594ba959377a2b904dc99eb5232593555ee4c21229713c2f3d8032eaf9a25e7f52a2929002b03e4c67c7fc8eca572478bd8e0dfc19b96ced75e74c18efa52bf04e96ce4efb3c16bae488c154a5cb36776d60c00b3ac5675cf7e2f162a4c6632eefe5738018ea1e52796da96435aa3d20e343732396aae26795a478bc573681322b71494de9856308c3c53da5803245ce6fdcdd4630af924ae50d219ae1af31ddc3c630459cd5b24d3191e473b97662f4d73b1bb45dff78a77487357d0b9778f7aadd599c8cffca506574a58f4067041ff453dbf059f616d0f48a5c24b5107518df1aa6ab76e2272ddf10944c031c2686607439ade35dfe75a655b65c98edddabc2ee5fdfc4a6a265c4adfc16858ba6d04bc28ac157f65669b06146774f5b6a9c784caa1ab8a18eb2aa98317f762fdbb2b63f8ec7d4493caadf878ef03f21b6c194798f1ba20998b022529c27ce7a65f2b46ae35e32aff360a2d3b57ebcf25efb73aa0e20019118507a0e1bbff962250dcc17de3a8000aca961fd15854bbc3633e068513d6f4277c9437523e8c1720aaf71ea2f42d2b96774a5f9d377dfe8a1fe7f35af47aa14287e2391689a72d114123eda718c28a40673867b6656e559d589e5883c06041a23eb7a9ff59292095ab8aaf3b4476945a987b98636d8bd291f61c518d0d42ded6830ed188b1b22f5e80302ad2d3d9f73722c4ccdd911fddc1db9365343249e5d50ef8ca54512ba6393f5ab9e88a4739f6dfd36f13010025c3a83106dc4658e4af52dc6ad1f06c5b5629229dcd6b34bbe364d27925e91d7d73add8cf0d0b073e1fd7a8dd12c8c0890ef2bf763ebb5f2fcde30239dbff3a54adc3376b2d5c094226b4b8dbef36771a4c5277fdf060ac78f34ebea0f26387a5e6173e515f8435f1bdb265cae3f1bd4fbffe51aadca5dce721618b45c377553437c40e198d2b3202e9173162c8855543dd95cfb603de78dab440fa90f6f5dda5695ac57e82fb4ad9f73237e24dda451de2def924a02ec5c76ab812a3dc1f1cbdca7fb47ae0c21d4f5a63c8df550a3b9ef079ce934c5ef6b79a602d4bf13e0a7eb8a7dc6d455a89d9820263575b621fe8f79c4b96554632f485a761a5435a127ebfc6f6738451134d642104da04c0f5394396c704ae17b5af2d4294981366469dff88923a6d3458305cd49ffe068730adc3b9d3766269cb2982c0587783a8f51731cf9ac1214e9767665c6cf722cb1503546650c5906e41b09691f944806f83e6d50b0dc244483b2f218311710b28a5b7c4a231be88e77f6299d967ac2c2efa60a1985d1a8af3afa98643521563e69ff69b39cbbfeef0a396294438185e2891de83ac1cef4860a446b62544f837e103f385891bee11d97cf436a4e7a6c434ea15655956e405f18d7c9d55d34f6d97a8fb575cd575ddd4f436049b57853f07281cc9f982d0842fe1a4434af218351f6da1cffa9ef897b270df787db06eb7883518233d135af822c0c44efad216c4bae59e121c3bdca0407061ef838c3ed3b1cbded4e04c12a3f2adc61578276c856d2c2c587bd34e80f5a5a60adfe051aa6570c9dbba7aca706cc7c342466a7d424c1e25a91357091d2da419f172b1bda486bd80e20a69b5194ca3824dea6677e6722114857c432e4567ca7832677e59715a0fae126306909ae67d093afdbe5a83bc874fc2831cabdc2d52de4edaa6c3f96543814ddced6f841f0fed3888590df0ecccd89837df6530dea97698f3db388425e7277cc888375a391c447b61c25122908180e14919922707259d41839ced627d99d9613f74c98195f24cd41f22d1493678eaa3e94819a9efd96e59ce8faaacf84215e1218661061c012326121157b338333c12356353dc659a78d514a6b85623eb17993b32404fc158ff2aea0b84b95a34b37fd6d47aec8e7c7a530317d20b4cf7b81621af1aeb45cdc7e3ad413b9d71330428e64980498a61fa066e8441a938c19d53fa5edc4a48b2c111d0e1551703d80f50ba4a68dec70e18c6deef7d685d666e68919bdf2db04e657dec294cd7e7e154ada98d456b0c41389635608b2ca290310d68536f23cb8db3883426bb582d883b3e00a514b673ab62f2b7f63efefa0051fd950da9363217e5e853c3f8bcd7ca8c5e33ba943a986188344458891233df41b77e1fdd5ffd6206b5729a807c0bddc214c38a567135cb1e8cb9a80f59e64187dba9cbb46568af7eb565b00b456edbf1d729b92e130306d7a360bae71c8a96d6abf2a0287738804378228abf390dd74c6ec39801d6b6b451dd514b1233fbf60e16daf3bcb733332db9c42a653925941f11215e862241b06ac0fc7b6ce2b24fbe8989f7f4dff6f49f5f3454fb8ccb983f57f4138a27a5b776348524c814819e189e263a42ae6bd77846c25814525076b9288094a833596a52d95040df0f0c19c8d07ea3fd518fdea11cac919ac9ce2bc22b6b65888db39dba4454ee4ee8140dfc03815a008dff5aac55e252ac6d6c8c0ec86d1396287fc5ccb";

// #[cfg(test)]
macro_rules! request_data {
    ( $rpc: expr, $( $fn:ident($( $arg:expr ),*)),+ ) => {
        {
            let mut temp_file = File::create("tests/json_data.dat").unwrap();
            let mut data = String::new();
            $(
                let req = Request::$fn($($arg),*);
                let res = $rpc.request(&req).unwrap();
                writeln!(data, "{}/{}", &req.method(), &res.result().unwrap().to_string()).unwrap();
            )*

            temp_file.write_all(data.as_bytes()).unwrap();
            temp_file
        }
    };
}

// FIXME: This should only contain stuff that can be serialized. If it Strings
// or else, I'm positive it'll be fine.
#[test]
fn generate_json() {
    let config = ClientConfig {
        username: USER.to_string(),
        password: PASS.to_string(),
        addr: IP.parse().unwrap(),
    };
    let mut rpc = Client::new(config).unwrap();
    request_data!(
        rpc,
        block(BLOCK_HASH, Some(3)),
        blockchain_info(),
        block_header(BLOCK_HASH, Some(true)),
        block_stats(BLOCK_HASH, None),
        chain_tips(),
        chain_tx_stats(None, None),
        mempool_info(),
        raw_mempool(Some(true)),
        tx_out_set_info(),
        memory_info(),
        uptime(),
        block_template(None),
        net_totals(),
        network_info(),
        peer_info(),
        decode_raw_transaction(RINGCT_TX_RAW)
    );

    // TODO: Manually generate data may not want to turn or may not have data at
    // time of running. mempool_ancestors()
    // mempool_descendants()
    // mempool_entry
    // tx_out
    // raw_transaction
    // test_mempool_accept

    // TODO: Those that need inputs
    // added_node_info
    // convert_transaction_to_pst
    // decode_pst
    // decode_script
    // finalize_pst
    // fund_raw_transaction
    // verify_commitment
    // verify_raw_transaction
}

// #[test]
// fn test_best_block_hash() {
//     let mut rpc = Client::new_test();
//     println!("{}", rpc.best_block_hash().unwrap());

//     // assert_eq!(
//     //     hash,
//     //     "8b8a64d49c71747430a2505f3a12e5058a354c1fc8d6b3d9fa80bd598b33719c"
//     // );
// }

// #[test]
// fn test_best_block_hash() {
//     let config = ClientConfig {
//         username: USER.to_owned(),
//         password: PASS.to_owned(),
//         addr: IP.parse().unwrap(),
//     };
//     let mut rpc = Client::new(config).unwrap();
//     let hash = rpc.best_block_hash().unwrap();

//     println!("Best Block Hash: {}", hash);
// }

// fn test_block() {
//     config!();
//     let block = rpc.block(BLOCK_HASH).unwrap();

//     println!("{:#?}", block);
// }

// #[test]
// fn test_blockchain_info() {
//     let mut rpc = Client::new().unwrap();
//     let blockchain_info = rpc.blockchain_info().unwrap();

//     println!("{:#?}", blockchain_info);
// }

// #[test]
// fn test_block_count() {
//     let mut rpc = Client::new().unwrap();
//     let block_count = rpc.block_count().unwrap();

//     println!("{}", block_count);
// }

// #[test]
// fn test_block_hash() {
//     let mut rpc = Client::new().unwrap();
//     let block_hash = rpc.block_hash(BLOCK_HEIGHT).unwrap();

//     println!("{}", block_hash);
// }

// #[test]
// fn test_block_header() {
//     let mut rpc = Client::new().unwrap();
//     let block_header = rpc.block_header(BLOCK_HASH).unwrap();

//     println!("{:#?}", block_header);
// }

// #[test]
// fn test_block_header_serial() {
//     let mut rpc = Client::new().unwrap();
//     let block_header_serial = rpc.block_header_serial(BLOCK_HASH).unwrap();

//     println!("{}", block_header_serial);
// }

// #[test]
// fn test_block_stats() {
//     let mut rpc = Client::new().unwrap();
//     let block_stats = rpc.block_stats(BLOCK_HASH, None).unwrap();

//     println!("{:#?}", block_stats);
// }

// #[test]
// fn test_chain_tips() {
//     let mut rpc = Client::new().unwrap();
//     let chain_tips = rpc.chain_tips().unwrap();

//     println!("{:#?}", chain_tips);
// }

// #[test]
// fn test_chain_tx_stats() {
//     let mut rpc = Client::new().unwrap();
//     let chain_tx_stats = rpc.chain_tx_stats(None, None).unwrap();

//     println!("{:#?}", chain_tx_stats);
// }

// #[test]
// fn test_difficulty() {
//     let mut rpc = Client::new().unwrap();
//     let difficulty = rpc.difficulty().unwrap();

//     println!("{:#?}", difficulty);
// }

// //{
// // "89727907a87458249ef6719987b9d29fb0a091141c2bf7b7c5c351f731429ea9": {
// //     "fees": {
// //       "base": 0.00000221,
// //       "modified": 0.00000221,
// //       "ancestor": 0.00000221,
// //       "descendant": 0.00000221
// //     },
// //     "size": 221,
// //     "fee": 0.00000221,
// //     "modifiedfee": 0.00000221,
// //     "time": 1574582128,
// //     "height": 457788,
// //     "descendantcount": 1,
// //     "descendantsize": 221,
// //     "descendantfees": 221,
// //     "ancestorcount": 1,
// //     "ancestorsize": 221,
// //     "ancestorfees": 221,
// //     "wtxid":
// "89727907a87458249ef6719987b9d29fb0a091141c2bf7b7c5c351f731429ea9", //
// "depends" : [ //     ],
// //     "spentby": [
// //     ]
// //   }
// // }

// // FIXME: Do when I have fake RPC server
// // #[test]
// // fn test_mempool_ancestors() {}

// // FIXME: Do when I have fake RPC server
// // #[test]
// // fn test_mempool_descendants() {}

// // FIXME: Do when I have fake RPC server
// // #[test]
// // fn test_mempool_entry() {}

// #[test]
// fn test_mempool_info() {
//     let mut rpc = Client::new().unwrap();
//     let mempool_info = rpc.mempool_info().unwrap();

//     println!("{:#?}", mempool_info);
// }

// #[test]
// fn test_raw_mempool() {
//     let mut rpc = Client::new().unwrap();
//     let raw_mempool = rpc.raw_mempool().unwrap();

//     println!("{:#?}", raw_mempool);
// }

// #[test]
// fn test_raw_mempool_verbose() {
//     let mut rpc = Client::new().unwrap();
//     let raw_mempool = rpc.raw_mempool_verbose().unwrap();

//     println!("{:#?}", raw_mempool);
// }

// #[test]
// fn test_tx_out() {
//     let mut rpc = Client::new().unwrap();
//     let tx_out = rpc.tx_out(BASECOIN_TX_HASH, 0, Some(false)).unwrap();

//     println!("{:#?}", tx_out);
// }

// #[test]
// fn test_tx_out_proof() {
//     let mut rpc = Client::new().unwrap();
//     let tx_ids = vec![BASECOIN_TX_HASH];
//     let tx_out_proof = rpc
//         .tx_out_proof(tx_ids.as_slice(), Some(BASECOIN_BLOCK_HASH))
//         .unwrap();

//     println!("{:#?}", tx_out_proof);
// }

// #[test]
// fn test_tx_out_set_info() {
//     let mut rpc = Client::new().unwrap();
//     let tx_out_set_info = rpc.tx_out_set_info().unwrap();

//     println!("{:#?}", tx_out_set_info);
// }

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_precious_block() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_save_mempool() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_verify_chain() {}

// // FIXME: Need tx out proof as `const`
// // #[test]
// // fn text_verify_tx_out_proof() {
// //     let mut rpc = Client::new().unwrap();
// //     let verify_tx_out_proof =
// rpc.verify_tx_out_proof(BASECOIN_TX_HASH).unwrap();

// //     println!("{:#?}", verify_tx_out_proof);
// // }

// #[test]
// fn test_memory_info() {
//     let mut rpc = Client::new().unwrap();
//     let memory_info = rpc.memory_info().unwrap();

//     println!("{:#?}", memory_info);
// }

// #[test]
// fn test_uptime() {
//     let mut rpc = Client::new().unwrap();
//     let uptime = rpc.uptime().unwrap();

//     println!("{:#?}", uptime);
// }

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_generate_blocks() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn generate_blocks_continuous() {}

// #[test]
// fn test_block_template() {
//     let mut rpc = Client::new().unwrap();
//     let block_template = rpc.block_template(None).unwrap();

//     println!("{:#?}", block_template);
// }

// #[test]
// fn test_mining_info() {
//     let mut rpc = Client::new().unwrap();
//     let mining_info = rpc.mining_info().unwrap();

//     println!("{:#?}", mining_info);
// }

// #[test]
// fn test_network_hashps() {
//     let mut rpc = Client::new().unwrap();
//     let network_hashps = rpc.network_hashps(None, None).unwrap();

//     println!("{:#?}", network_hashps);
// }

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_prioritize_transaction() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_submit_block() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_add_node() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_remove_node() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_try_node() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_clear_banned() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_disconnect_node() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_added_node_info() {
// //     let mut rpc = Client::new().unwrap();
// //     let added_node_info = rpc.added_node_info().unwrap();

// //     println!("{:#?}", added_node_info);
// // }

// #[test]
// fn test_connection_count() {
//     let mut rpc = Client::new().unwrap();
//     let connection_count = rpc.connection_count().unwrap();

//     println!("{:#?}", connection_count);
// }

// #[test]
// fn test_net_totals() {
//     let mut rpc = Client::new().unwrap();
//     let net_totals = rpc.net_totals().unwrap();

//     println!("{:#?}", net_totals);
// }

// #[test]
// fn test_network_info() {
//     let mut rpc = Client::new().unwrap();
//     let network_info = rpc.network_info().unwrap();

//     println!("{:#?}", network_info);
// }

// #[test]
// fn test_peer_info() {
//     let mut rpc = Client::new().unwrap();
//     let peer_info = rpc.peer_info().unwrap();

//     println!("{:#?}", peer_info);
// }

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_banned_nodes() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_ping() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_add_ban() {}

// // FIXME: Do test for this on fake RPC
// // #[test]
// // fn test_remove_ban() {}

// // #[test]
// // fn test_combine_pst() {}

// // #[test]
// // fn test_combine_raw_transactions() {}

// // #[test]
// // fn test_convert_transaction_to_pst() {}

// // #[test]
// // fn test_decode_pst() {}

// // #[test]
// // fn test_decode_raw_transaction() {}

// // #[test]
// // fn test_decode_script() {}

// // #[test]
// // fn finalize_pst() {}
