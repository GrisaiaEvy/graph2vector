use fastembed::{Embedding, TextEmbedding};
use graph2vector::embedding_strategy::{StrategyBuilder, StrategyFunc};
use graph2vector::graph_db::GraphDbFunc;
use graph2vector::graph_db::neo4j_graph_db::{Neo4j, Neo4jParams};
use graph2vector::vector_db::milvus_vector_db::Milvus;
use graph2vector::vector_db::VectorDbFunc;
use graph2vector::vectorization_service::fastembed_service::FastEmbed;
use std::{env, io};
use std::io::Write;
use config::{Config, ConfigBuilder, File};
use lazy_static::lazy_static;
use log::{info};
use graph2vector::config::RUST_LOG;
use graph2vector::llm::LLM;
use graph2vector::llm::open_ai::OpenAi;

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new(/* ... */) -> FooBuilder {
        FooBuilder {
            bar: String::from("X")
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn builder(self) -> Foo {
        Foo {
            bar: self.bar
        }
    }
}

#[tokio::test]
async fn builder_test() {
    // let milvus = Milvus::new(String::from("http://192.168.20.218:19530")).await.unwrap();
    // milvus.insert(String::from("1"), String::from("content1"), vec![0.8869797985362167, 0.4020722617804955, 0.16477795681850593, 0.9340520668174104, 0.31231791282443777, 0.31054162214775194, 0.5196887876407286, 0.8414555923594942, 0.9839281633056727, 0.8169525850840877, 0.34822135599898085, 0.2875296779039511, 0.8074854454204581, 0.7809260193140595, 0.008745794826095388, 0.553631816258281, 0.3007787184186048, 0.7200090256954419, 0.05053245660858585, 0.20399779105419458, 0.9210755616023021, 0.9543487036770926, 0.04745961154905931, 0.32656129260359346, 0.7361341583540726, 0.654615912816239, 0.17959895269745285, 0.8115790214746659, 0.5921176027857422, 0.7573270321420533, 0.6670371278741041, 0.8108632139452554, 0.3414080376696538, 0.4760989998829679, 0.7491236409434652, 0.04473075891306344, 0.5914256739526893, 0.5107817793432601, 0.38586920100610467, 0.6815631754877125, 0.024467443555576285, 0.7143571520461702, 0.2874213425516965, 0.6339047389110906, 0.7001350340135291, 0.6971767707759529, 0.5677506402684738, 0.1269619056207374, 0.13097475805827208, 0.8278312473187532, 0.774146819613101, 0.6990446630922968, 0.988658015504146, 0.07380370657629354, 0.7730394797870304, 0.7143484809762952, 0.38869512860377853, 0.4530579627716882, 0.704377608446535, 0.3618992096972049, 0.03789757251190773, 0.16869932330337534, 0.9840715944015683, 0.0776039078451487, 0.8357547809240935, 0.185484994818218, 0.6145776993862182, 0.90308348327952, 0.9487341085997263, 0.9776722116384113, 0.241115952407702, 0.880432504500462, 0.8147724924078859, 0.575528220573928, 0.944677555045004, 0.43836999811633937, 0.04717392380678609, 0.40021718776198933, 0.21150080685750594, 0.31083978701521575, 0.5424028576472726, 0.5529620986332431, 0.2671315983001994, 0.7448111135886684, 0.1748119397257748, 0.9687059398690727, 0.8808144602226569, 0.9302952255380474, 0.12325907537742031, 0.7064518059783433, 0.23716778674994998, 0.3917577174279234, 0.54921636934956, 0.08863842412074341, 0.9370700669459187, 0.6208212858701592, 0.7312783944521546, 0.21168447468481388, 0.5964870553995998, 0.13838035517101188, 0.760449108411323, 0.8097760894109127, 0.7511132721142952, 0.7139689595262908, 0.19616225581891222, 0.5919606776367747, 0.519682661027574, 0.39805525140704323, 0.48885495857933825, 0.13879981417816967, 0.011145515585174737, 0.8212454455135871, 0.7149500576290932, 0.2029682802064745, 0.9750300037715054, 0.6627835499864545, 0.2370862274078207, 0.4732628968668795, 0.9749023597787432, 0.6598062703226515, 0.39843907555417823, 0.04789409165758807, 0.17691377132881492, 0.6863325306833061, 0.8722548921161637, 0.7006743865208052, 0.5973132797422447, 0.5575727992907307, 0.8883627603047746, 0.7262297664941417, 0.5548766649112942, 0.07143964138152792, 0.5662561714195806, 0.6869995805327664, 0.2889179544864011, 0.36458179144570657, 0.12148058960688801, 0.28354907541310803, 0.3263013381649802, 0.1059607268523699, 0.9705880511611669, 0.9132838298478623, 0.1381787298799675, 0.038789959386240636, 0.8689771838340996, 0.8666236448335489, 0.05896747704214533, 0.6975322218247844, 0.7691175311005818, 0.2553354758566351, 0.7899529112097581, 0.3380352100044546, 0.6594736819973335, 0.7117344478292029, 0.014592065176199709, 0.3715016742109307, 0.2538001669451515, 0.25518946289890243, 0.35664658009799366, 0.8985559922273787, 0.2539350969224057, 0.2912256631747161, 0.10582531271845008, 0.8278213154820562, 0.5366847820679361, 0.2492832417358255, 0.1806339735839384, 0.09232902820424038, 0.5709193552524896, 0.38191078801446876, 0.6068900119253131, 0.0996814831688062, 0.563215979824579, 0.43167113101650645, 0.8993589992515507, 0.8281885921642655, 0.9086926628766945, 0.2664769257316195, 0.20376033743051303, 0.1174399173155869, 0.9818821928449588, 0.9246623562557228, 0.35891560280995005, 0.24428874713821447, 0.7908342700038589, 0.22765789121714586, 0.48353607020301403, 0.4626375696998808, 0.3618320603990175, 0.6914577017544625, 0.4909376576765132, 0.24629818816955917, 0.05839410997891292, 0.4667296737971969, 0.0426631511121951, 0.1017828470179325, 0.6548272032655489, 0.4346093921601619, 0.5796310140332828, 0.7863810297276699, 0.5702181816069036, 0.6735027976157282, 0.22120750192411687, 0.7889796929402677, 0.9108110780090193, 0.40229741026381527, 0.6282886485603076, 0.04170010077320141, 0.847533756192093, 0.6716893285786125, 0.4385098234165221, 0.08557102751806167, 0.27015091504559785, 0.9786053496840095, 0.6032936749331177, 0.8641950775914198, 0.12705140179017183, 0.1904636386313212, 0.06372880939450076, 0.4716659002837771, 0.689693551685916, 0.8235867148726541, 0.979932617639361, 0.8513466054492145, 0.9886064372119192, 0.5589441861710842, 0.07853727326531867, 0.9772748207379951, 0.22923419838987869, 0.6352906684109163, 0.42472969960026785, 0.47453888849573866, 0.9758692887024809, 0.001480162508768812, 0.8623057778240641, 0.22953008412701248, 0.471749260451515, 0.43700673331018214, 0.628399696095397, 0.38407756450428754, 0.673740211050678, 0.8915324407058316, 0.7782502842492842, 0.5686846611012129, 0.197263883808672, 0.15091580398536553, 0.6967448771472171, 0.18065008276775085, 0.27966414204744927, 0.10111694795703041, 0.5126243667940273, 0.32843217943077385, 0.5003307271886046, 0.7043314428573795, 0.7118124376937758, 0.09891690087687888, 0.9115565149993048, 0.07634698017928976, 0.18270347457547254, 0.24925184366333575, 0.2818888840893803, 0.27514662667658163, 0.052020431978106574, 0.3893084714812405, 0.13283959635282128, 0.6770127923099081, 0.9744991614314966, 0.7422695058285653, 0.7115106514396905, 0.3226445143664074, 0.39659124669952983, 0.3411568851620421, 0.036847142385843634, 0.6276669495677418, 0.4455983470306317, 0.7411622325905385, 0.762047535149794, 0.6260701263952064, 0.5616123585901958, 0.9926670573705119, 0.036751207288036936, 0.6652862267358324, 0.7962455046042234, 0.21993951210475982, 0.8224595939237096, 0.03587757234062661, 0.6362050145358653, 0.3261384929292821, 0.9219614405415795, 0.9043673691278753, 0.6265880669982531, 0.05553272825598765, 0.8581561399318929, 0.5320246656007455, 0.6372631716683252, 0.9439848487845757, 0.17618483284795028, 0.2398550369939787, 0.5074305020310572, 0.9390540481041547, 0.6791535449994439, 0.06676713222862873, 0.7116981795900537, 0.11000857455275925, 0.24182845293990374, 0.6233422450363137, 0.7052723276279413, 0.6274281197708396, 0.15333833700196542, 0.550245647789172, 0.3167030785970042, 0.040963511774739336, 0.49173941023293555, 0.5193951071289562, 0.4936805402177529, 0.7740273758247422, 0.8526980141419331, 0.42649786977968307, 0.0899052174659174, 0.26264348304237295, 0.6436880756810612, 0.04975985516167869, 0.13855869092631035, 0.994285442540676, 0.5399736574803515, 0.3142563275794943, 0.6604202330810067, 0.6517510859903448, 0.10806537623704826, 0.9333167465492986, 0.11690508232814345, 0.8657225587505952, 0.11873899311341218, 0.18333708359045708, 0.09684600320268877, 0.05846848749589317, 0.8970559777668743, 0.4961460327352196, 0.11008340424490748, 0.2413758481416084, 0.06864156595620696, 0.00036468184999205455, 0.6202678155610775, 0.11442572045319133, 0.9963026626777429, 0.8511213341950463, 0.5680750270910879, 0.3639319043810141, 0.8435204158148171, 0.9769551087005057, 0.39288977230390665, 0.7764971667295242, 0.3020830075437777, 0.9425526237294992, 0.45003960243044006, 0.45299660485988835, 0.8544450823659964, 0.7711570766435052, 0.5292933698104854, 0.9483592141528137, 0.515985601436038, 0.5372925993800677, 0.5725612188852678, 0.8329875236995075, 0.9902851256456713, 0.04332114926902464, 0.28155502608533567, 0.3514766718582736, 0.10332623684835829, 0.8371722203905596, 0.656239679018265, 0.027009900320307922, 0.0011618888697308982, 0.7727759794554279, 0.9356019692995574, 0.3152702426934879, 0.08296843340784554, 0.47529914795803374, 0.4041063840921695, 0.7830747173450923, 0.2857276080780462, 0.03381926286014747, 0.22664314300431632, 0.9329993559299155, 0.3641060040866082, 0.22734183532210994, 0.21160534824395794, 0.14825632140299572, 0.10316777681664036, 0.8607697355444961, 0.24765538858701341, 0.3989778184880539, 0.2874126512741837, 0.04730226270588833, 0.19067042565441938, 0.2561689006535075, 0.13689807910206775, 0.8738919185286163, 0.9216833223175598, 0.9180070009434398, 0.011340142269104181, 0.7538410616662012, 0.7430224996830046, 0.1677550469699105, 0.17830547382323858, 0.6185608622883001, 0.4654676638744353, 0.5800024853262817, 0.6432769633799214, 0.36076181221185455, 0.7976899392710544, 0.6112101134340737, 0.3163821700498519, 0.5588988380435522, 0.6750115692274483, 0.04758038368956341, 0.17324406501278, 0.8434600790995759, 0.034760707674940994, 0.7829917778592916, 0.8503294108584605, 0.9283605797465329, 0.3168876641897158, 0.4488266475101943, 0.5855453862452424, 0.739244741502886, 0.773360601225308, 0.8657438333112739, 0.21788134407785842, 0.9193521743812749, 0.22483347592692748, 0.03259233461833411, 0.03930003972041907, 0.7021466129945797, 0.9157748612343855, 0.583368873500806, 0.8597403332539655, 0.8254024507731363, 0.8890665680004886, 0.999554969003861, 0.3631853165619585, 0.761014387846247, 0.773469489144587, 0.1523031155495176, 0.3604904099886992, 0.11011796003142194, 0.8669563073368685, 0.9156270230423718, 0.6296756969025501, 0.5151837806767319, 0.37694576507588406, 0.5522742788256785, 0.1900783617581061, 0.5358147867415815, 0.8399487056402528, 0.3846304697704277, 0.2970861166182597, 0.6592329175793712, 0.0961233360382272, 0.768499548031842, 0.6431441868237311, 0.639697404510114, 0.7471667193467482, 0.027881992817446077, 0.4589527608343851, 0.1598207180878728, 0.2027938880447464, 0.005823982177260323, 0.02203026004325026, 0.9362001323400326, 0.7256410512106877, 0.245858479571188, 0.3785389549241682, 0.4362546437063515, 0.4946910673417828, 0.22866167664308734, 0.9862410688509289, 0.6431565742647267, 0.7997849678526652, 0.697927696019252, 0.7262627817575704, 0.6839849522195818, 0.2601899301775832, 0.2738693559628471, 0.12463427867539845, 0.43710425033800915, 0.633625247623768, 0.323181465004724, 0.49925798889013473, 0.6825117486484622, 0.9836594064967901, 0.7078029962043793, 0.34550477124991286, 0.3242345742199584, 0.06059742204045149, 0.5850623124210157, 0.6416161436982624, 0.8185620159111924, 0.2812580158007434, 0.8973888737214932, 0.20406932827328683, 0.6274155904984857, 0.9158164853308031, 0.9913485903045698, 0.880390759023993, 0.8347545606561331, 0.6403791416610789, 0.9259072073434746, 0.24656968655601252, 0.035982114788619235, 0.9145406317955034, 0.12501713780537305, 0.39530607274520246, 0.16799884205467563, 0.830668044548827, 0.021351573436547033, 0.4664648355605392, 0.2865324296426468, 0.28874688961379635, 0.595485877596075, 0.30025382213276, 0.24118136292015602, 0.6387415914355894, 0.3702976410004535, 0.5914178388922284, 0.07259021477121097, 0.7949017652072214, 0.7566520089030024, 0.8375347688087533, 0.07312512648604996, 0.5414744980199178, 0.5676980062610446, 0.15995945931659006, 0.1751008729707415, 0.5970747009734112, 0.3899911455167, 0.9563531949470065, 0.16313180270423855, 0.061656215621583854, 0.6691457691022347, 0.37428462287947983, 0.24123141605573384, 0.7584841997531706, 0.2200931048522008, 0.3910535119510481, 0.4440859770031751, 0.4941989682562524, 0.9604151225385702, 0.08474893817821916, 0.7518130690322857, 0.2873291504998048, 0.9391388267169374, 0.05419369983973188, 0.6305078700466848, 0.6237644195062284, 0.9689694376603346, 0.20990101346483403, 0.9804859416093845, 0.6365113762717374, 0.3252373571117493, 0.13260465786057063, 0.08188037866514719, 0.3794283066641817, 0.5374932271901642, 0.3456524241624399, 0.08677432305414312, 0.8702436166387664, 0.4163500826890756, 0.7466557079336287, 0.11107036930899161, 0.7587637163047196, 0.8365743199694744, 0.4427906746520458, 0.4736839375401356, 0.8449213282822594, 0.41259107392462524, 0.4709162457711309, 0.52036781056948, 0.34664638574494, 0.9453361255780302, 0.33780002443750967, 0.9415524403780546, 0.04062350703592821, 0.9101331325106825, 0.690014877466756, 0.20160978090679516, 0.26115319220625377, 0.5999565016552011, 0.6557889565264443, 0.456721340591405, 0.40670344004984216, 0.9953023056626211, 0.7295865025806998, 0.4699458164476751, 0.9646799808785222, 0.07290358658532825, 0.27714689201070697, 0.923255767674172, 0.053352910562751266, 0.0037325476407161773, 0.056475163978078946, 0.7462132803746604, 0.5280417457090294, 0.08867563987045468, 0.8483664135857112, 0.8754277450527141, 0.9330462030748361, 0.719042743940447, 0.09610942673629852, 0.3784391583917297, 0.4735007292122704, 0.27202002539193715, 0.731873337078141, 0.5569332169475558, 0.9546891160602702, 0.675139884523386, 0.1790150950559215, 0.7982447507667794, 0.8211847784467698, 0.7296461633722422, 0.6598331630700256, 0.5639385592631565, 0.3804425419369173, 0.4133945011557254, 0.9287828092168953, 0.7156574964276052, 0.6838906956304034, 0.10216975305571685, 0.9711856011626729, 0.45805663987096645, 0.4420386538651735, 0.7376153137267603, 0.9330644771109469, 0.5997971051937128, 0.7131011196705532, 0.3175471417113531, 0.7215880600242754, 0.3008844428505728, 0.8409136696597661, 0.07757956237185959, 0.5164600911894552, 0.5629978216203446, 0.5871350180103407, 0.46793621109909345, 0.4654943452587905, 0.9345897359913555, 0.047534177197305016, 0.6108229752660728, 0.4121482344181986, 0.28168949380205177, 0.16282403850437, 0.67393538970859, 0.9139072209877803, 0.458451293496009, 0.8532784315047512, 0.07259282363567499, 0.12614952432014315, 0.12026323941188055, 0.418754736749821, 0.24765268462215673, 0.9152924707252152, 0.17418694199540652, 0.11880038337909804, 0.6879120482574892, 0.39963993813609155, 0.019639471230561867, 0.6458581416502376, 0.5015721471113068, 0.714255759449943, 0.9583500343870353, 0.5603972435802316, 0.8918904803914391, 0.7192336112409141, 0.08656254894872006, 0.06893691400689272, 0.04868424496770429, 0.025722516363243297, 0.2916107477000751, 0.5771024076059135, 0.08645484560554473, 0.27204832445144267, 0.884430729753537, 0.5060801684172946, 0.8320513623277006, 0.9170453242485916, 0.9278644110503393, 0.05017018858786537, 0.5600356499363479, 0.4524560134477511, 0.3744105861177778, 0.6186450358125373, 0.19953405231731214, 0.5716945226629968, 0.6445009734896918, 0.5810781281416288, 0.09473909833337135, 0.8836641827198088, 0.6948459500382789, 0.7240680356227382, 0.24759974697257792, 0.2433666849741758, 0.0637941441876082, 0.399511013354376, 0.9123676565984382, 0.6110146137607753, 0.884458850865989, 0.485612631924043, 0.1242854008080283, 0.17390803202075023, 0.7573777375356474, 0.7638616780363607, 0.5717147068136625, 0.5984804373615455, 0.36885074759063863, 0.391246633349267, 0.26658230949235073, 0.7946729564459818, 0.03926366817264948, 0.09031842206237961, 0.8649379911342525, 0.011114452346253412, 0.5227359954271795, 0.4583220418402858, 0.8074426940629098, 0.9414530149138811, 0.6642578345405505, 0.5502656511121538, 0.5198102759907395, 0.9841211969672214, 0.5207214335402175, 0.7222640245542415, 0.7198181775996455, 0.9466558634162758, 0.7669631905330245, 0.9768787380800767, 0.32856195158040635, 0.5815143859188947, 0.672172835477612, 0.20736659217848064, 0.953384092584717, 0.6272494055266031, 0.790813996895759, 0.7983260931188165, 0.7891708371076531, 0.5681776512284733, 0.813262289045882, 0.9276440945683926, 0.9300809890596002, 0.9586387826321114, 0.3826479290249496, 0.38779830553285866, 0.15600322023705382, 0.14669025619902198, 0.9131178761030816, 0.9431554364231094, 0.5601201764451451, 0.8740438976524503, 0.8972385772869655, 0.28669505709876675, 0.10585602736047828, 0.5056904344992081, 0.5590298275662202, 0.06825460450156573, 0.7834604191396404, 0.03236613498432872, 0.09611461696373746, 0.2775918653998759, 0.3753904400274215, 0.3665534830064823, 0.9181530283602357, 0.250254149917267, 0.9248308277989801, 0.19798989523803834, 0.31629592859539146, 0.7074055482208101, 0.9041340363135391, 0.7474113615383222, 0.6114476642777931, 0.6416944896764858, 0.6295634032773021, 0.7141577842377256, 0.8640871992333818, 0.783553557210499, 0.2536949316564303, 0.712760000525845, 0.5328218335674766, 0.7757557595750793, 0.6149909289102018, 0.6650491886091627, 0.2648726054431527, 0.0941758088693676, 0.04616907984886787, 0.15767290982889515, 0.5440954755853957, 0.12165018738723132, 0.168894380013195, 0.8988744998430862, 0.7990646370161332, 0.07695737161029581, 0.5824680673544369, 0.23790082069269314, 0.7306021540195728, 0.75821748653361, 0.6937771777327946, 0.11630325684358067, 0.545724172104536, 0.5519801505074733, 0.014601709915873728, 0.7662905803185525, 0.3307214468898412, 0.9837816261440593, 0.07011195947729498, 0.3712595352579897, 0.8478462211575271, 0.8705293353958705, 0.9734345768735426, 0.4121949913530416, 0.8887278378051413, 0.2878120856630504, 0.6225610027574506, 0.7231524511858711, 0.8857928237483619, 0.8681231168423662, 0.8640366652100442, 0.8033338555462199, 0.7065529397048593, 0.5316991737831651, 0.01867082353108529, 0.9935464200165931, 0.6054004879650359, 0.48068310305227446, 0.9937611965339679, 0.6783335398907357, 0.023406903371672394, 0.9929159090194337, 0.6284579463256348, 0.09271577440305911, 0.40814290900550665, 0.8887745869700574, 0.11316234944893044, 0.3849780102957152, 0.859955624829897, 0.14709909028158674, 0.5079576352286088, 0.5333661060634995, 0.2719689976018693, 0.14648400639080372, 0.9970074444352834, 0.6391437621106846, 0.010339780711568602, 0.5800670633404559, 0.24293490169355025, 0.288127885922846, 0.03498023702929842, 0.8703356534110227, 0.3259399557910134, 0.2810519683659962, 0.6776284845451366, 0.5745038360343064, 0.7563526629454416, 0.8795392157547823, 0.3612316236550732, 0.35978792668088655, 0.6628470019549317, 0.22170648603585574, 0.6236637899886572, 0.03854986220468448, 0.48332952605637747, 0.24303070284514083, 0.7131738602740001, 0.399737111730494, 0.12651156664062757, 0.40610577247670254, 0.8811705885487215, 0.18458803423620873, 0.19078588393037776, 0.08927524522880126, 0.9665778658752464, 0.05030213915587822, 0.3765571422897154, 0.7867389423702218, 0.8482534215559381, 0.5525190370125364, 0.253209123629375, 0.9950046777293386, 0.5231688299769188, 0.8343685487340391, 0.3970653069290171, 0.8388413668165666, 0.4100390504469591, 0.3353562260156733, 0.02866163592255022, 0.9849845984288359, 0.7765550900903748, 0.946770642280526, 0.49210563740307856, 0.44069075559026194, 0.8927070990390304, 0.210630372957876, 0.010486939828308062, 0.4567034559695866, 0.956324161300153, 0.9467911060702026, 0.6880823031959118, 0.11087255093441839, 0.4960411576191057, 0.5646557743091283, 0.46087713332547087, 0.33345862486150013, 0.4784797853139269, 0.8645341333215251, 0.8206371614845132, 0.7816713784646301, 0.34501448383531974, 0.09107309910469175, 0.8922335707666444, 0.6565862126581925, 0.5690756877042153, 0.9008976067886769, 0.29507174926062696, 0.8694140257891236, 0.6560125588003007, 0.9381749077249213, 0.7454081666936747, 0.6963448509280454, 0.18138958653295867, 0.612925172210109, 0.28842270156280003, 0.5428951232570238, 0.3459141979120217, 0.3751406308722931, 0.4681885612077008, 0.14141466649340662, 0.16924518008360634, 0.6970241950189995, 0.754493277530397, 0.07176618326258621, 0.7050071242302707, 0.11328205240035238, 0.056546169803924906, 0.7739391128290276, 0.3048369317853026, 0.6367403469780224, 0.36808103539298975, 0.04165271160807804, 0.582670659631747, 0.9358797595299964, 0.5141913858788167, 0.6054630236621019, 0.810787062542923, 0.4349075826577484, 0.7921739118503373, 0.39547392755122135, 0.2393620697269907, 0.7311026916480992, 0.9621359632030184, 0.05692663973167922, 0.29014703007734544, 0.8075414348541481, 0.4611461101775649, 0.057568754088022756, 0.4746010311904514, 0.9750258393339062, 0.3472426943491973, 0.882788568970799, 0.1339178808605559, 0.47196385049915834, 0.5757801281225774, 0.8786547236872837, 0.9714687518931229, 0.46730201690797757, 0.905410338592507, 0.813411861807996, 0.28055375075473044, 0.9833895472524812, 0.06189402365478225, 0.3248241726559282, 0.23036913854356933, 0.1058397827558899, 0.24648703586898701, 0.4414066472118241, 0.25229228220972, 0.8149437760985425, 0.00020522635013842105, 0.590337428987185, 0.7760075710340055, 0.6395195658543658, 0.5158542250589173, 0.4835668040679151, 0.33689053821222936, 0.14451885644367524, 0.039532479305654844, 0.9602469034495451, 0.5021110173662917, 0.6826461384066138, 0.6464373521580549, 0.3135409783448315, 0.5386404380496956, 0.46414824549431355, 0.4783220911723671, 0.22047295645795684, 0.8816467093711478, 0.9925539679154249, 0.19023814010302487, 0.7859967982440654, 0.11330352934133292, 0.23824666850067922, 0.35078698260548635, 0.9520658201319065, 0.3623573736779233, 0.28788732308619824, 0.4196078172443012, 0.3272113134691965, 0.2212377786861619, 0.9206721190531251, 0.4535661521633896, 0.5870796488149463, 0.20012257314642978, 0.05758639229367413, 0.952852106843719, 0.22488448245280646, 0.7422320193119687, 0.6970610869506608, 0.6356526235073672], String::from("metadata1")).await

}


#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", RUST_LOG.as_str());
    env_logger::init();

    info!("begin");
    let neo4j = Neo4j::connect(Neo4jParams{host: String::from("127.0.0.1"),
        port: 11003, user: String::from("neo4j"), pwd: String::from("123456"), db_name: String::from("***")}).await;

    let embed = FastEmbed::new_zh();

    let milvus = Milvus::new(String::from("http://127.0.0.1:19530")).await.unwrap();

    let ai = OpenAi::new(String::from("https://***"),
                         String::from("****"),
                         String::from("gpt-3.5-turbo"), 0.5);

    let strategy = StrategyBuilder::new(neo4j, embed, milvus, ai).build_entity_strategy();
    // strategy.load_data().await;

    strategy.launch_ai_cmd().await;
}

