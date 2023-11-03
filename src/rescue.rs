

use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;


const PRIME : u64 = 2147483647;

pub struct Rescue {
    pub width: usize,
    pub capacity: usize,
    pub rate: usize,
    pub n: usize  
}

impl Rescue {
    pub fn new(width: usize, capacity: usize, n: usize) -> Self {
        let rate = width - capacity;
        Self {width, capacity, rate, n}
    }

    pub fn hash(&self, input_sequence: Vec<FieldElement<U64PrimeField<PRIME>>>) -> Vec<FieldElement<U64PrimeField<PRIME>>>{
        let m = self.width;
        assert!(input_sequence.len() % self.rate == 0);
        let mut state = vec![FieldElement::<U64PrimeField<PRIME>>::zero(); m];
        let mut absorb_index = 0;
        while absorb_index < input_sequence.len() {
            for i in 0..self.rate {
                state[i] += input_sequence[absorb_index];
            }
            absorb_index += 1;

            self.permutation(&mut state);
        }
        let mut output_sequence = vec![];
        let mut squeeze_index = 0;
        while squeeze_index < self.rate {
            for i in 0..self.rate {
                output_sequence.push(state[i]);
            }
            squeeze_index += 1;

            if squeeze_index < self.rate {
                self.permutation(&mut state);
            }
        }

        output_sequence[..self.width].to_vec()
    }


    pub fn permutation(&self, state: &mut Vec<FieldElement<U64PrimeField<PRIME>>>) {
    
        let alpha: u64 = 5;
        
        let alpha_inv: u64 = 1717986917;

        let m = self.width;
        let n = self.n;
        let r = self.rate;

        // round constants 
    let round_constants: Vec<FieldElement<U64PrimeField<PRIME>>> = vec![
        FieldElement::<U64PrimeField<PRIME>>::from(1419697373),
        FieldElement::<U64PrimeField<PRIME>>::from(1814085569),
        FieldElement::<U64PrimeField<PRIME>>::from(1979784298),
        FieldElement::<U64PrimeField<PRIME>>::from(1325652769),
        FieldElement::<U64PrimeField<PRIME>>::from(723038292),
        FieldElement::<U64PrimeField<PRIME>>::from(191878156),
        FieldElement::<U64PrimeField<PRIME>>::from(1328810909),
        FieldElement::<U64PrimeField<PRIME>>::from(209425547),
        FieldElement::<U64PrimeField<PRIME>>::from(312130129),
        FieldElement::<U64PrimeField<PRIME>>::from(163897463),
        FieldElement::<U64PrimeField<PRIME>>::from(457385638),
        FieldElement::<U64PrimeField<PRIME>>::from(482437588),
        FieldElement::<U64PrimeField<PRIME>>::from(798462392),
        FieldElement::<U64PrimeField<PRIME>>::from(1909553168),
        FieldElement::<U64PrimeField<PRIME>>::from(1872984146),
        FieldElement::<U64PrimeField<PRIME>>::from(1648772291),
        FieldElement::<U64PrimeField<PRIME>>::from(1178214426),
        FieldElement::<U64PrimeField<PRIME>>::from(1381154135),
        FieldElement::<U64PrimeField<PRIME>>::from(2042080178),
        FieldElement::<U64PrimeField<PRIME>>::from(1840952828),
        FieldElement::<U64PrimeField<PRIME>>::from(412209816),
        FieldElement::<U64PrimeField<PRIME>>::from(1016054564),
        FieldElement::<U64PrimeField<PRIME>>::from(674682305),
        FieldElement::<U64PrimeField<PRIME>>::from(1461198792),
        FieldElement::<U64PrimeField<PRIME>>::from(1594276761),
        FieldElement::<U64PrimeField<PRIME>>::from(859863871),
        FieldElement::<U64PrimeField<PRIME>>::from(1570565042),
        FieldElement::<U64PrimeField<PRIME>>::from(447248457),
        FieldElement::<U64PrimeField<PRIME>>::from(930102061),
        FieldElement::<U64PrimeField<PRIME>>::from(1697310138),
        FieldElement::<U64PrimeField<PRIME>>::from(197185462),
        FieldElement::<U64PrimeField<PRIME>>::from(861339850),
        FieldElement::<U64PrimeField<PRIME>>::from(2107189013),
        FieldElement::<U64PrimeField<PRIME>>::from(1727523537),
        FieldElement::<U64PrimeField<PRIME>>::from(1342045701),
        FieldElement::<U64PrimeField<PRIME>>::from(2004564379),
        FieldElement::<U64PrimeField<PRIME>>::from(1760230909),
        FieldElement::<U64PrimeField<PRIME>>::from(779564668),
        FieldElement::<U64PrimeField<PRIME>>::from(428115779),
        FieldElement::<U64PrimeField<PRIME>>::from(1675449035),
        FieldElement::<U64PrimeField<PRIME>>::from(2007274509),
        FieldElement::<U64PrimeField<PRIME>>::from(283236679),
        FieldElement::<U64PrimeField<PRIME>>::from(1008633409),
        FieldElement::<U64PrimeField<PRIME>>::from(1535258290),
        FieldElement::<U64PrimeField<PRIME>>::from(932530555),
        FieldElement::<U64PrimeField<PRIME>>::from(1195184341),
        FieldElement::<U64PrimeField<PRIME>>::from(508634104),
        FieldElement::<U64PrimeField<PRIME>>::from(1280406383),
        FieldElement::<U64PrimeField<PRIME>>::from(31953700),
        FieldElement::<U64PrimeField<PRIME>>::from(1613243890),
        FieldElement::<U64PrimeField<PRIME>>::from(259646794),
        FieldElement::<U64PrimeField<PRIME>>::from(207496054),
        FieldElement::<U64PrimeField<PRIME>>::from(684317779),
        FieldElement::<U64PrimeField<PRIME>>::from(625703579),
        FieldElement::<U64PrimeField<PRIME>>::from(698071139),
        FieldElement::<U64PrimeField<PRIME>>::from(1838670163),
        FieldElement::<U64PrimeField<PRIME>>::from(557141195),
        FieldElement::<U64PrimeField<PRIME>>::from(1583568521),
        FieldElement::<U64PrimeField<PRIME>>::from(1351465890),
        FieldElement::<U64PrimeField<PRIME>>::from(1314050840),
        FieldElement::<U64PrimeField<PRIME>>::from(366841122),
        FieldElement::<U64PrimeField<PRIME>>::from(1366825094),
        FieldElement::<U64PrimeField<PRIME>>::from(903565198),
        FieldElement::<U64PrimeField<PRIME>>::from(907421172),
        FieldElement::<U64PrimeField<PRIME>>::from(969301921),
        FieldElement::<U64PrimeField<PRIME>>::from(1716495553),
        FieldElement::<U64PrimeField<PRIME>>::from(1982340421),
        FieldElement::<U64PrimeField<PRIME>>::from(630705869),
        FieldElement::<U64PrimeField<PRIME>>::from(439948723),
        FieldElement::<U64PrimeField<PRIME>>::from(788008546),
        FieldElement::<U64PrimeField<PRIME>>::from(857368082),
        FieldElement::<U64PrimeField<PRIME>>::from(1661759228),
        FieldElement::<U64PrimeField<PRIME>>::from(948584746),
        FieldElement::<U64PrimeField<PRIME>>::from(1687816905),
        FieldElement::<U64PrimeField<PRIME>>::from(924185651),
        FieldElement::<U64PrimeField<PRIME>>::from(23662964),
        FieldElement::<U64PrimeField<PRIME>>::from(1911401458),
        FieldElement::<U64PrimeField<PRIME>>::from(244559968),
        FieldElement::<U64PrimeField<PRIME>>::from(1270656357),
        FieldElement::<U64PrimeField<PRIME>>::from(1119906711),
        FieldElement::<U64PrimeField<PRIME>>::from(439495046),
        FieldElement::<U64PrimeField<PRIME>>::from(1122462938),
        FieldElement::<U64PrimeField<PRIME>>::from(1736663253),
        FieldElement::<U64PrimeField<PRIME>>::from(939571234),
        FieldElement::<U64PrimeField<PRIME>>::from(843273521),
        FieldElement::<U64PrimeField<PRIME>>::from(1826381148),
        FieldElement::<U64PrimeField<PRIME>>::from(1822070494),
        FieldElement::<U64PrimeField<PRIME>>::from(457291586),
        FieldElement::<U64PrimeField<PRIME>>::from(849666679),
        FieldElement::<U64PrimeField<PRIME>>::from(1542728225),
        FieldElement::<U64PrimeField<PRIME>>::from(871651129),
        FieldElement::<U64PrimeField<PRIME>>::from(95390807),
        FieldElement::<U64PrimeField<PRIME>>::from(512373444),
        FieldElement::<U64PrimeField<PRIME>>::from(1293733091),
        FieldElement::<U64PrimeField<PRIME>>::from(1561670539),
        FieldElement::<U64PrimeField<PRIME>>::from(1290350939),
        FieldElement::<U64PrimeField<PRIME>>::from(1513940110),
        FieldElement::<U64PrimeField<PRIME>>::from(386792288),
        FieldElement::<U64PrimeField<PRIME>>::from(2040927415),
        FieldElement::<U64PrimeField<PRIME>>::from(192012609),
        FieldElement::<U64PrimeField<PRIME>>::from(1465214234),
        FieldElement::<U64PrimeField<PRIME>>::from(1505585262),
        FieldElement::<U64PrimeField<PRIME>>::from(574986702),
        FieldElement::<U64PrimeField<PRIME>>::from(52073732),
        FieldElement::<U64PrimeField<PRIME>>::from(449616296),
        FieldElement::<U64PrimeField<PRIME>>::from(162728909),
        FieldElement::<U64PrimeField<PRIME>>::from(1115569236),
        FieldElement::<U64PrimeField<PRIME>>::from(110396085),
        FieldElement::<U64PrimeField<PRIME>>::from(143980421),
        FieldElement::<U64PrimeField<PRIME>>::from(588269760),
        FieldElement::<U64PrimeField<PRIME>>::from(1279005750),
        FieldElement::<U64PrimeField<PRIME>>::from(1894006967),
        FieldElement::<U64PrimeField<PRIME>>::from(1470377657),
        FieldElement::<U64PrimeField<PRIME>>::from(1345609680),
        FieldElement::<U64PrimeField<PRIME>>::from(1463099209),
        FieldElement::<U64PrimeField<PRIME>>::from(1682059476),
        FieldElement::<U64PrimeField<PRIME>>::from(1377747969),
        FieldElement::<U64PrimeField<PRIME>>::from(677924114),
        FieldElement::<U64PrimeField<PRIME>>::from(732148535),
        FieldElement::<U64PrimeField<PRIME>>::from(1644426071),
        FieldElement::<U64PrimeField<PRIME>>::from(888343064),
        FieldElement::<U64PrimeField<PRIME>>::from(1712347472),
        FieldElement::<U64PrimeField<PRIME>>::from(1526486259),
        FieldElement::<U64PrimeField<PRIME>>::from(1833782480),
        FieldElement::<U64PrimeField<PRIME>>::from(1936240142),
        FieldElement::<U64PrimeField<PRIME>>::from(1562122056),
        FieldElement::<U64PrimeField<PRIME>>::from(1771264340),
        FieldElement::<U64PrimeField<PRIME>>::from(1211765669),
        FieldElement::<U64PrimeField<PRIME>>::from(843492656),
        FieldElement::<U64PrimeField<PRIME>>::from(1765932204),
        FieldElement::<U64PrimeField<PRIME>>::from(1403681963),
        FieldElement::<U64PrimeField<PRIME>>::from(41315677),
        FieldElement::<U64PrimeField<PRIME>>::from(1528766465),
        FieldElement::<U64PrimeField<PRIME>>::from(2090967878),
        FieldElement::<U64PrimeField<PRIME>>::from(903563037),
        FieldElement::<U64PrimeField<PRIME>>::from(1916462472),
        FieldElement::<U64PrimeField<PRIME>>::from(1602372601),
        FieldElement::<U64PrimeField<PRIME>>::from(1987115013),
        FieldElement::<U64PrimeField<PRIME>>::from(1424091239),
        FieldElement::<U64PrimeField<PRIME>>::from(2017406519),
        FieldElement::<U64PrimeField<PRIME>>::from(2126892694),
        FieldElement::<U64PrimeField<PRIME>>::from(1616453582),
        FieldElement::<U64PrimeField<PRIME>>::from(1824501492),
        FieldElement::<U64PrimeField<PRIME>>::from(1049190072),
        FieldElement::<U64PrimeField<PRIME>>::from(818256022),
        FieldElement::<U64PrimeField<PRIME>>::from(1674250680),
        FieldElement::<U64PrimeField<PRIME>>::from(1986950393),
        FieldElement::<U64PrimeField<PRIME>>::from(804099344),
        FieldElement::<U64PrimeField<PRIME>>::from(1069661541),
        FieldElement::<U64PrimeField<PRIME>>::from(1869861745),
        FieldElement::<U64PrimeField<PRIME>>::from(1276857766),
        FieldElement::<U64PrimeField<PRIME>>::from(860373131),
        FieldElement::<U64PrimeField<PRIME>>::from(2014618202),
        FieldElement::<U64PrimeField<PRIME>>::from(571246841),
        FieldElement::<U64PrimeField<PRIME>>::from(1928732397),
        FieldElement::<U64PrimeField<PRIME>>::from(1551065111),
        FieldElement::<U64PrimeField<PRIME>>::from(941509089),
        FieldElement::<U64PrimeField<PRIME>>::from(573418399),
        FieldElement::<U64PrimeField<PRIME>>::from(268521559),
        FieldElement::<U64PrimeField<PRIME>>::from(377399742),
        FieldElement::<U64PrimeField<PRIME>>::from(1971407831),
        FieldElement::<U64PrimeField<PRIME>>::from(944317878),
        FieldElement::<U64PrimeField<PRIME>>::from(1763076689),
        FieldElement::<U64PrimeField<PRIME>>::from(320105269),
        FieldElement::<U64PrimeField<PRIME>>::from(1670292887),
        FieldElement::<U64PrimeField<PRIME>>::from(512030705),
        FieldElement::<U64PrimeField<PRIME>>::from(198702587),
        FieldElement::<U64PrimeField<PRIME>>::from(1079440265),
        FieldElement::<U64PrimeField<PRIME>>::from(866570219),
        FieldElement::<U64PrimeField<PRIME>>::from(1548668136),
        FieldElement::<U64PrimeField<PRIME>>::from(1544135299),
        FieldElement::<U64PrimeField<PRIME>>::from(878358755),
        FieldElement::<U64PrimeField<PRIME>>::from(1604814372),
        FieldElement::<U64PrimeField<PRIME>>::from(709505469),
        FieldElement::<U64PrimeField<PRIME>>::from(597132039),
        FieldElement::<U64PrimeField<PRIME>>::from(1354019305),
        FieldElement::<U64PrimeField<PRIME>>::from(1607569343),
        FieldElement::<U64PrimeField<PRIME>>::from(1187853244),
        FieldElement::<U64PrimeField<PRIME>>::from(585730297),
        FieldElement::<U64PrimeField<PRIME>>::from(336004002),
        FieldElement::<U64PrimeField<PRIME>>::from(1453923046),
        FieldElement::<U64PrimeField<PRIME>>::from(1280906971),
        FieldElement::<U64PrimeField<PRIME>>::from(1469190295),
        FieldElement::<U64PrimeField<PRIME>>::from(218982689),
        FieldElement::<U64PrimeField<PRIME>>::from(583638321),
        FieldElement::<U64PrimeField<PRIME>>::from(20391668),
        FieldElement::<U64PrimeField<PRIME>>::from(1178172181),
        FieldElement::<U64PrimeField<PRIME>>::from(69036119),
        FieldElement::<U64PrimeField<PRIME>>::from(1127405736),
        FieldElement::<U64PrimeField<PRIME>>::from(626064118),
        FieldElement::<U64PrimeField<PRIME>>::from(164309891),
        FieldElement::<U64PrimeField<PRIME>>::from(1566500094)
];
        

    let mds = vec![
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
        ],
        vec![
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(9),
            FieldElement::<U64PrimeField<PRIME>>::from(7),
            FieldElement::<U64PrimeField<PRIME>>::from(4),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
            FieldElement::<U64PrimeField<PRIME>>::from(16),
            FieldElement::<U64PrimeField<PRIME>>::from(2),
            FieldElement::<U64PrimeField<PRIME>>::from(256),
            FieldElement::<U64PrimeField<PRIME>>::from(128),
            FieldElement::<U64PrimeField<PRIME>>::from(3),
            FieldElement::<U64PrimeField<PRIME>>::from(32),
            FieldElement::<U64PrimeField<PRIME>>::from(1),
        ],
];

// The matrix mds is now initialized with the provided elements


        let zero = FieldElement::<U64PrimeField<PRIME>>::zero();

        for _ in 0..n {
          for i in 0..m {
            state[i] = state[i].pow(alpha);  
          }
          
        let mut temp = vec![zero; m];
            for i in 0..m {
                for j in 0..m {          
                    temp[i] = temp[i] + mds[i][j] * state[j]; 
            }
        }
          
        for i in 0..m {
            state[i] = temp[i] + round_constants[2*r*m + i];  
        }
           
        for i in 0..m {
            state[i] = state[i].pow(alpha_inv);
        }

        let mut temp = vec![zero; m];
            for i in 0..m {
                for j in 0..m {          
                temp[i] = temp[i] + mds[i][j] * state[j]; 
            }
        }

        for i in 0..m {
            state[i] = temp[i] + round_constants[2*r*m + i];  
        }

        }

      }

}







