/**
    Day 1: Trebuchet?!
    https://adventofcode.com/2023/day/1

    Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
    You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
    You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
    As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
    The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

    For example:

    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet

    In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

    Consider your entire calibration document. What is the sum of all of the calibration values?

    --- Part Two ---
    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
    Equipped with this new information, you now need to find the real first and last digit on each line. For example:

    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen

    In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
*/

fn byte_to_digit(byte: &u8) -> Option<u32> {
    (*byte as char).to_digit(10)
}

fn find_first_and_last_digits(line: &str) -> [(u32, Option<usize>); 2] {
    let bytes = line.as_bytes();

    let mut first: u32 = 0;
    let mut first_index: Option<usize> = Option::None;
    let mut last: u32 = 0;
    let mut last_index: Option<usize> = Option::None;

    for i in 0..bytes.len() {
        let left_byte = bytes.get(i).unwrap();
        let right_index = bytes.len() - 1 - i;
        let right_byte = bytes.get(right_index).unwrap();

        if first_index.is_none() {
            match byte_to_digit(left_byte) {
                Some(val) => {
                    first = val;
                    first_index = Some(i);
                }
                None => {}
            }
        };

        if last_index.is_none() {
            match byte_to_digit(right_byte) {
                Some(val) => {
                    last = val;
                    last_index = Some(right_index);
                }
                None => {}
            }
        };

        if first_index.is_some() && last_index.is_some() {
            break;
        }
    }

    [(first, first_index), (last, last_index)]
}

fn combine_two_digits(first: u32, second: u32) -> u32 {
    first * 10 + second
}

const DIGITS_STR_LIST: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn map_word_to_digit(word: &str) -> u32 {
    for i in 0..DIGITS_STR_LIST.len() {
        if word != DIGITS_STR_LIST[i] {
            continue;
        }
        return (i as u32) + 1;
    }
    panic!("Should never arrive here");
}

fn split_and_clean_input_into_lines(input: &str) -> Vec<&str> {
    input
    .trim()
    .split('\n')
    .map(|line| line.trim())
    .collect::<Vec<&str>>()
}

pub fn trebuchet(input: &str) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();

    let lines: Vec<&str> = split_and_clean_input_into_lines(input);

    for line in lines {
        // if by coincidence the first or last character
        // are digits then we can save precious time
        let [(first_digit, first_digit_index), (last_digit, last_digit_index)] =
            find_first_and_last_digits(line);

        // since no number string length is longer than 3 characters
        // we can assume that if the number we found exists within the first 3 characters
        // it's really the first number (and same logic in reverse for the last)
        if first_digit_index.is_some()
            && first_digit_index.unwrap() <= 2
            && last_digit_index.is_some()
            && last_digit_index.unwrap() >= line.len() - 2
        {
            numbers.push(combine_two_digits(first_digit, last_digit));
            continue;
        }

        let mut first = first_digit;
        let mut first_index = match first_digit_index {
            Some(index) => index,
            None => line.len(),
        };

        let mut last = last_digit;
        let mut last_index = match last_digit_index {
            Some(index) => index,
            None => 0,
        };

        for digits_str in DIGITS_STR_LIST {
            let matches: Vec<_> = line.match_indices(digits_str).collect();
            for (index, value) in matches {
                let digit = map_word_to_digit(value);
                if index <= first_index {
                    first_index = index;
                    first = digit;
                }

                if index >= last_index {
                    last_index = index;
                    last = digit;
                }
            }
        }

        numbers.push(combine_two_digits(first, last));
    }
    return numbers.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet 
        "#;
        let result = trebuchet(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn it_works_with_words_and_digits() {
        let input = "7z";
        let result = trebuchet(input);
        assert_eq!(result, 77);
    }

    #[test]
    fn it_works_with_puzzle_input() {
        let input: &str = r#"
            eight9fhstbssrplmdlncmmqqnklb39ninejz
            three656
            ppjvndvknbtpfsncplmhhrlh5
            7fjqhrhsevenlbtwoninevnmct2
            qjnbpfrztwo1
            plggqjthree49four
            xpxpbsdc1
            three2six8two5
            36two
            five121zvjks3
            4onefive6zsjhzvrjnsfive1
            six5vfb
            5h9bnkjfivemlqkf
            fnxqvsvqbzxgkfour5
            eightjzqzhrllg1oneightfck
            6threeeightjzcgsnclfive7txvgsdxnt
            sevenninexsjtjvcsixtnfivejhj45
            6lxpdpdnr
            56cnfourgrkfzxcvpsqd
            gmlqzxdxtt9five
            oneninepnvtfbbcx98vmttscj64
            lptxklsh17three2
            znzzthreecmpbtc3kbmjmfxtczjhd476
            threeeight16three
            cc8one
            jvlmlfvcxk2zxkpxvsf
            foursevenjqncdtkqxg65hcqxrssvlq
            five359hlgblsthree
            lbfrhccstlhrrzones2bdeight9
            nzzsixrxzvbkzlh9vxrkmnhkkkfvffzdmphx8
            ninedxxzv7vnthmltpf5tthree7seven
            8ninesix8monehbvmvrvsrvsqmhmxz
            four1qpntfzmqnkkcnv
            gpxprxzl15five
            onebprckgrrgj2
            sevenjqkc14qfkcjfs4three
            4onetrsnzpsixtwo7rxkdfh
            seven4kmrlprpz
            33twohsevenone67
            gbzm97nineslblhnine
            kchtbjlg37three
            four83dmpzflg
            5eight1qrd
            hqdtv8vcqqrth4mvcbtgfrbtninejkrfzvbfkq
            3jrrjhtt91sixfourfive
            seven9four
            1twospfczxstwogcrxhcrjs6
            6onetjvskhmlrcnxnmgrh
            chkpkzpqnchfrplmphhncsrppmx4ftwo1six
            htr7
            dlxhmzsbtthree5nine281eight
            35d9
            bfhoneighthkfvgbhbqsthree4fzgbcpdgseven
            tntvhfvlone1thqdfivemfrfourfour
            2kxshspv6llkpsixjpfppncthree4
            two8one
            fvcbsevenlkthree32tdbpfive
            56nine9one8fourcpnine
            xzcgvrk35shf
            8smrmnsseven
            4onedxlxflgfcj
            1gxvgxrqxzone
            gvzkmxg55twonem
            4threeone9cjjrvvqx
            srpnvmpgm1sixone
            five5seven3twozmfmqmsstpthreessdnlmssk7
            seven1nplbftheightfourgxflgone
            eightthree77kqxpppninensjqcsnkz32
            xxmdgdstkzmgfpz4
            jfdpcgvjnnbsrkvninefive45six
            qqvsc1983onertkvkhjtj
            mngm7threed85pq
            15one4nine7hzbgvdmtwo7
            xnthree3843
            35threeninetwo3
            gmc7sgfqjkqxlpgbjkpcrbeightnine8fbhgp
            ctjhrmnsevenone6xlzvtjk
            14xzgsbvrleightfourseven3
            8nqxtj4ninehhvsbfxnpl
            45prcggcvqpmone2tdggkfgrrvfivemcspfnf
            cnoneight8rdbdjvjbseight
            pvkxgtfx2hmkznp9six
            8cbzksc
            tbzcfourvrfsdrjx2threestxdqsdcnp
            two1zkmprx
            5fninenkzkclksbfourvclcdnkmrx
            28four4
            two2qkxlctcndmqxm7
            sixthreekzdvkpmptvkrxpzdthree16eight3
            6six85k
            72four
            4one8tf1fxoneone
            6twod9six2
            sixsix3onepqnjspdmxfour1
            fqqcmvgkdfnvjschmtnzplpkl1eight
            fourqtrtlsix2
            5vknkpb5825ninetjzsr6
            1oneeight53eightthreeonenjk
            7vbktk1onethreezhnthqhncc7xbcqmqgtg
            fourfivevxfbbbtgfpbq4nine5
            four32two
            foursix4jlhkzqcgg
            3cbrftrphh8twoeightfoursix4
            7onegqhngfcsrrlknk97six
            9prrrtgoneonekmrhxlttwo8
            2jpfqlnmrjprxbs8hhtgxg
            8tmckkdlgtmzljlsq
            fivedznqszq67kc7onetwo3
            pdjjdfive434jzrqngcz
            ljmjtwoldvkddzpght456three
            1krvcrn
            hndmzcqfour75zsxrlfourthreenine
            277lcjzvxhv33llzrddkb
            seven8vv
            2fourqhkmtdh
            pvk9four
            eightqqgpncmncmsjgvkpxqzdjvkqjhsninexsbkjtk6
            gvfsrxpp59
            sevenseven7three
            jpcxldqbmdptnlj838rcpftjmfgonekmfltkrdlg
            95threevktfour5lzntgmfbcdx
            mhdzxhmfqgffjhjfh5k635three6
            6bmbrcznrbsgfzbpmnjdtt16four
            rnbcjrzthree9four5
            kcjjnxc9nine4tr1seven2
            hdmpcjzvs4eight1
            7gtbjdkfnrnfive
            five3qkhmtpfldvfourphfxvjssspfourtgctwo6
            cfkr8seven99zxvhxrzx
            fourshjqd1six
            sixceight7
            plxhflcmqsgjpn5mhsphvv
            three5ninehntgrlchcnnmqx98two5
            8sixcbqlfmcq14vnlmsixlhzrq
            4eight2dsdnpnsx
            fivexntprmkhpronejbnbseighttfnzmkdn3six
            ninejtrqtneight6rvtnqspmkjsix
            jsvgpb4
            tgrtn2qsbjsjjhxnntcmmgxrfivenmchb
            rtvlmjpgbhmjxsixsix3ninetwonr
            4jpkgvzzcnine8one
            1threefivexzcqlgpbmczvvkrcznbr
            dvgf6fivezvpvvz5vvtdhqpj4
            five8threesqqzlfvl1
            oneeight7onenine1
            lngrtsrfive6fcfhmfgxrc
            eightfourtwoplzjkclh6
            2six9
            threefour5
            8twopmonedgn
            3eightg
            onetwoktxtbhffxvnhjbmbjqrgsh29tqlkpcnine
            8sixoneninetwofive7qz5
            nine5skfsmrzeight61five
            four1two8zvhcfgdpsb72
            4seven2six5
            fivebsix81two9xmjq
            nxnrsdsixeightgfbmmhhst2251eight
            nine339ksh9seven
            sixonesixjblvfqvftxpjznf5fivefour
            fvnvvp6twofive25twoneg
            fourthreebr8gnkmrh6llrlg2four
            kjjgzq25four19
            zlsfbnfkm64five24jtqmvgjtrzggqnfive
            8nkrfltkeight
            eighttlzrsgbfjp8
            sevennfrmfscs8four5czrlrxhmxl
            threeeightrdnqvrdd9
            7six2824
            ggteight8nine23
            twofour3sevenqvzn6one
            4rdcmnhtrdfour
            3j515
            onecgfhxvndbfqkcsbbksix2mbszjp95
            3tk61ltpcg
            fivefourdnine1
            86vjxfmvbtqgnflmgrnp87kqsgt3
            qlcshkxbxtvflbqth5sxsxlj9zvchrgqzdmlmtmsfldb
            9twothree7bfrqjgztdlmrdxzpbdnd1
            psrfcqqbs62lfhthree4
            v3dhzkznrkzfgjksgppzxgpone
            svqnineqkjtfxvbff2rqkbvqctzh1n
            2sqvnkmvjlqflbv1654twoeight
            twoseveneight56trvvzv
            5qp
            cgrks12857
            ltflnbzslg8fkfourkllzh
            1four9eighteight8nine6gbdv
            three9eight2vcndvvkphllzkqfbvbc
            2eight1fivefcmdcbdn5onedffr
            jqbnzzjxjhrvdzbglgtnrgccrhxjqtnrqt9
            sixhdggzgsrr8five8zjndfskgbr2five
            9dsclqonefournine
            eight8qvnpcmzkgd37srgrvseven
            7gzc124kninetwoseven
            pnfthree46foursevenfivek
            6bsvbrrfoureighttwo
            sixsevenfivefhrslvvdmf2
            5eightczxjsgfjpmzdrzkg6
            rzvtzjvmdvr6five3
            six2sevennttwo
            2one1
            klfour13four
            8xdnmzonesevencldxvlhxmnjpcfgnqfourfive8
            8nkmkqgngfourpnddbmtvnfour3h
            9154
            crddnxhhznnkcptjcxds8sixvjr7fllrj
            three9dfkzjqxbxfive1fhqzmlpgtseven
            seightninepjr3mjkgq3ckxzlqkkxpxdpkk
            four2flfsvhlm3vnbktkhjbj
            four2ndrspfq8gmseven766
            4one5two
            993
            1eightworc
            rzpfczninefive8
            5seven3bjrsgqdncnpbxqstqfnine23nine
            one2xdstrtwovpponefngmjh5
            foursix72psvxtwocxxz81
            845vpjdhdd5four3hhv
            eightthreefive828ssxktxqrsz3
            48six62two1
            5bfhqfbqsmjndthreenxlzfhlhz2
            4onehgsix75six
            5zjbqr6fivefourrmfvf8
            threesixfive1qnmbv4six8vxrsjzkd
            threeeightthreelxprspvcone926eight
            four3phmjpmmq3one
            1nvszzgtp795n
            four4eighttbmzcjkfbzlvzgjnsg
            nsmlqsixfiveng65jjblflfone
            xzcdfpscgm869s9
            sixsixgrndhrfnpthree4
            tgtpqllvsr4rhx8nine6lgdfx
            lrkf9sklkonezszgc
            11fiveeightdmqldjvjfivelbqdhbpbgbtwo8
            3ndfour
            293
            bfgrxvjprnine4fourbnxltjzpgnseven
            622fxlbpsmm9seven9tkppzxfc
            6pfkrrpdltqzghp1
            1twozfvskmlzrjpxnnj
            nmhht3four2
            41threeninesixdbvrnshnmfsx21
            sevenkzvmsfpzkbx2pzfvdttjbtwo51nine
            dbhjz1nine
            4cbsrqdk6hzxtmld
            xbvqzsltsvckzgzqgj47mdlcqjone2bbl6
            gtpmrrgrdmsfrfvrddqsbgrfmqlvnjmpd5gbxpzgcm6
            9t8ddhjgvfmtwo
            2ninefivebvlgtrj4two46five
            six4frshtwomqjmq
            8four488
            45nxxhvbqlfivefour
            eightnine36twogzqcnine
            751sdpjqvtvgvznsm
            863cgcg
            3rhrpdgmds1
            twoonevpzhjddskcqfive3ztqxch6
            583one2four
            5fczvhfqhrd39gnpmbvcnvvhsbnj9
            jthreefivezqbncdmcvhjftkpccsjzlv6three
            six51
            4nineseven1861ltvljzhneight
            one1kjbsm8nxsjs2mnzn
            pdqkjrbxs69bgpm8six8
            8qhfhsn8tjtwofivesevenonexpjtjm
            2r
            fpdfcndph7sixnine3fm4two
            fourl2seven5threepjonesix
            8three6sjxtthree4sixfkmnthree
            tscm83vpnrclcf
            6onecvgfshgcnznines
            pthreeqrcsgvtnpsix5
            eightvlqdjcgpzkbxcjzjhdsbvkcv2skcspgcqtpsnncxrthree
            sevensixqp3
            9eighteightckpkjbn
            lthree2jfncnkzcfq3
            8two64rrqprsj
            twonineglpjqmkbdc7kkcz5five
            threefourvrdgpzcn2
            sevenlknine4
            one346sevennkknkts
            7z
            32bjldfrccj9onejdgrhmtvm
            five72threesevenrnfour
            11sevengvjvhk
            173
            fcmznseven9three9nvbbcglpjk2
            lmdjlpxbg777ggpftspzjlmh118
            sixeight9threenine4
            qsix2
            three6sixeightlbgfbdzlf8threesix
            vvblfthreeeight2msjkqjjtnfpkgqgkcxthh8
            chhoneightone9lkczftxeight8
            threeninehmqddfsxjxvknvsgljp841
            1tfgk673
            4four3qcsix6dqnt
            gzrgcgt9threebxprzqfmgz6rdtgkbvjv
            nflvtzljs7kgbpfourcvxptsz
            blgdmqtdlphtjbfsix7seven1
            eightpfdblscrtf9seven
            pvkl7cbfqk8mflrxhvhgvfourthreeqrdlxhjxp
            threeqbqjdhgkkfour61
            fbjgmrjhtjl2twonsq
            sixthree357snkvbcr1
            79zxxsix2lvtwo5
            five51
            twothree4eight4one3
            6gsxlggspbbgsixsixsj995
            vtzdkqxksthree9fourfive
            392rklpf4mfqcrtxsixfivexfcszd
            one56bxzkqcsixsevenkqclbfvvh9
            8onefivetd1nine
            3dsckcnqfive1sixkrc
            8djmrcpzeightzkkspjq
            dnbxzn6four7hsevenpsdbfgztjtckvlc
            fpmrqk3
            3one71seven
            qkdcshhbsshcvslhteight4tgskjnzvone
            fourgtqdqxgxtqpn7jlgq1eightsj8
            1tkhxbqmdvthree7jvcqtbnsixeightwor
            8nine5rvckdfqf
            3hkbthreexfxtbszrcneight
            1fivesixthreenine
            4eightfnhzdtxkcsthree24
            55mhpsxtscpk
            lbhfsnineone8
            33sixnlctccbs3
            one1jt879cthreeeightwob
            1nine8ptwo
            21threexffczhhv
            eight113544pg
            dfvlpdknmlqqf3cllbnzzbp8nlfpbqtfive
            two1sfbltjbkbneight28rmzpcgn8rmkhmrjj
            48zvcqcb
            eightntddpqxgfkzrpcsmx68
            tdlppdpjsix59six
            1lqtsxd3
            4ggpbhzhxlsxmhshkkqlqgncsjrtxfpfztdp57vntmdlrdhvjr
            94grp9
            stvxsmdcflthree7gxncrthree5
            kjfbdzxztgfourrj13
            3three8dxffkzfjsevensevenseveneight
            vnkhcsjfmmpsixmf9djntprkccp
            nine5277
            rjktlvdfcvvkvtd8
            five9jtjhncc7
            fivenkfzczg2b
            hbd2qphbcpjbjqtwo2lbsvv25
            sevenstsfoureight48bjcbsixchlrjxfh
            9zghjpvlssdb
            fneightwoonernrmnvvleightfour6jfmznzlsq9
            8ctvt3oneseven6zzmjzgv
            xzeightwosixqxxcszqlfthree2threehpfzzzgmk1one
            pcxcmnmeight2three
            53lkpgqmtccnvsqfpn323
            dbfnnmcpone1sslqtnsv2ninemttk
            ffjppdlpkbgmlbfivenine2bgcqdzfps
            twoseveneighttwomhsztbhnzz9phsrjvnpg6nine
            xfqsevenfourxk3onefivesix
            one33
            4zthreexlgonevzlgxq
            six9cpvrshgjzqvqslttwo
            six1three4one8mdgfqc6
            cpvmpgltvpntwo855
            vroneightone8n7
            ntwoneqkdjznjfour5ppfp
            bzlqxfive9onesevenhztwo76
            pt6148
            2ninegvlqckrvqzhh
            sevenhr8sixgkcncfxkssfoursmgmk
            tmmjbjqnine7xjtjltkksgzgthreesix
            fivefive5lzpctqk7
            9fivelglghkvbnzeight29cql
            7five5ninemnxzjzfrfn
            786onehnfm11
            331vpeightthree3
            8fourfive87pf53four
            7cqlsqtjjcfour5bdjcxrvnmdthree
            15two27
            5seventwoonextsmpeighthjlcssevenone
            tqfzvfzstsixqczklsd2jdjphblsvrnine5eight
            4jlnqmmpvg
            eightszlmmnzhjv2six
            seventhree92fivelpkqvqspjsix
            oneeightfiveptx4fourone
            tbxcm95zzp
            rfqd56
            twogkbj5phrgktgrpqqqzvhcdfzq2l2d
            3slsrhqrfdcgjjpsxqcqppdkkzptbzq
            six3ninesixsix
            8nine9threethreeshrv48
            bvfivetwo9rfourbgtjthpdj9
            lngsxhnineone616one
            sdvzzvclvkgxseven6rvncxzdrdjhhkqfcf9mlnzxntg
            1zdd8
            2bnfnb1qhqfour77
            ninefourthreenine7
            four376seven
            1txdvzfourkqbjkhone
            34jcdcbgjlhqblsccd
            nkkcdlkzscxgqjl7732nine
            4nsnhsevennine1pvbsmeight
            35ddqnp1h
            nkmtp7
            jvrgthqv58bvthree
            nzzdvhqd65rk
            xvln2eightjqlfvn7cmfjbtrmc6
            six1cpfnzvzsqkvgkbkfvjxsix2
            onedzfnmndj47
            btrnfptfivetwofourxvbeightdqqt8
            vdbjckztd6
            4sevenfive72oneldjmnvjcmheightfour
            one886jr
            nxsvxtqtnm99onefive2xmttx3
            6threebrcrtxhgsixbrhlhnczpdbmfdgpxbcone
            fjsxkdeight8gnvgnmp995
            86eightdcdfvgcv7trq
            9812
            ninezgstkmnmzfzmglcfour981
            pbsqbrqgsvqrqmnjddsvrttnnlbgq932
            two2hbc
            sevenzktpspbhnlgseven2ztd
            svonefive57fourdmgjgzhk2fsgkczq
            onexglmkkgc4
            tctceightfive2leightxzbpddbqtwo
            nvzeightwojjjsftdgv56fiveeightnqpqr
            395onethree6
            fk5sevenonefive
            nine7four8seven
            1nineqqrldthree8fptsrdpfnqnine5h
            two8six958seven
            dgtnb8fourgbnine
            6zvhqbsthreeninesixtwonep
            94cjtzhqckqxlk8
            233fiveqcfcjdfxh7two
            srqchthreekztwo749
            16ldgone
            zfvtspcd55fivetwo
            twoonesevenonesix77
            xchhlxk9fqcqmnhddstlj
            5sevenkljjjpseven9fourone
            92rjbjpptmzxfjv5nine
            qs6eightdgkbrlconetwo
            6lv
            ddqpfnxzdhhb8
            3twornpckh6ninetwo
            4czdfkeightfjqtbeightgndbfourjnznxrjvzx
            sixeightrqvrqvvzxqflfztxs8six1
            qxfzlbftcsixvgr2mjfpxsbfour
            1mvbthree8kxgbpmc8qcnlkbknhr6
            ssfhr84
            twosixninecgxdbm3
            seven9eightwoj
            pxtdmcdff3qvfkzqtftwo5
            sevenxptzflgpgt7fdmlsptx
            twoglqdjtslz5
            jgjdrhcmghmlfszdfcsbl1dlbnfblrjncdl
            2onetqftxthree26csixeightoneightpxt
            threefoursix49
            pslnpxonec44fourninesix
            two6nrgbldxz6one
            lmrtwonertzlhstnvvqgz1seventwofive8
            pphxjgvhdthree1
            6qdcjd2twonetb
            ninecxgvsdmmlpfxhhzfcv2threethree7rsxpdnd
            5qg
            66455
            vb86pxgvnzshxhthree
            5vhgtwoone3
            four16
            eightxbjbzjbl1qhnhqtgq7tjtlcseven2
            zqstwo8one23three9four
            71fourrnzg6
            1qmhz
            fiveseven7
            fourmcqnine3sixz5
            36bfourvxbsmzqrmneightsfntqfdcgsqghpmv9
            7967
            fourtwo2four9fivekcvsf
            seven9fourgcngrdbmbvhgmnkvn4
            328kkjbqpvkjg
            onethree57
            72twolstbbfvstmqdtsdz1
            2fourone6three
            dczthhhjqzrsfour3
            three8xqdtp6threeone
            twothree8jdrpbjrxgxvkqkpbsfhglb1
            3dnkvl8
            foursix2cbkrzvlbtqqsdbcd7mkzbmzthree
            99onethreezghkkj
            four4rhrltfthree1
            four6fiveshd
            cvdlnfkns1mlnz5
            15onezjnm4fmvdhtzsgeightseven
            five6tgtlbqmkxdnmqjsn28one94
            vc3bnblfpvdxs8sixlftxr4
            cpbgvklmplj8cbnnmgzzmh1cxqvnn6
            67njgvqpgthreesbfdjcgtgvtwothree
            742
            97oneninehbjzfgbdjr1vvns
            mprzgsqtwo7
            stbjfour8rt
            dxchvbm2onercjfggjnine9tnfxzdklhpx2
            bdtwone9fourqdlhsfmstwo
            nxvcq4cgxhcn19vhkznxrbmthree
            bcd1
            5tktfnjxtwo
            7six12eight5bgxqjgclk
            vrfgxxfivevmjllxhbcgzgqlzmnfxhpnvtjp6nine
            n5eight
            onefgnz1gfmblblgvl7seven
            six95threeninevdmxpjvpzkpc
            6eightpbjftrzthreekgptrlrrxlkhcpsmq
            kvsmgkk5ndlbqrhb
            xjnrxxnpxmx3
            bpgkvznj1jrlhkfive
            jmh4sevenxchsh25
            fivejhtfhmrcfjfour18hmxzg5
            71vrbbbtxsjrxcxdztwo
            threesevenfour7eightthreegznpcdkpqjvpklrrb
            4zrjhgnmvshfour4
            2foureighteighttwoseven
            cmxfqjqvtb9
            eightlzkbn91snmckvgtgjxlktlzjpp
            nine9kprhsbnsjonenspsldrsf7xzxcm
            ktcrnftwo1cnnbdfqbng89onerhbrjd
            dvbzrrqninesixfxmxmxgfntgvkzsnqs5
            dbmfqhkxz6eight
            srxftlk2fivehzhzjlgreightxxrmbqghc2
            5qdgbxd
            fourthree3four9nvfivejsoneightc
            6ktzrdxnnd
            5seven3zlqzslrvmfj83nine
            5nine6fftx2shsslcgseight5
            91two
            qlkfvkdbsr1seven4
            6onemgbvthreec9three43
            fdssixnine5cvjrbcggxnhskngtd46
            three9nine49
            two2vxmghcjjjxmninemvninebl
            41nine6eightthree4lgtccg
            6cfivetbdjjrpz4
            lmctwone734
            eightrstfklgsdcthreehcxfnmmsc45five3
            8eight93sevenrcqtxzxgpn
            foureightpsnjhmtzfr1
            ksixthree21
            2hchsgvmssmrvrtwo8
            kfpgb791sevenseven21
            fivekzzh9
            52three
            fivetwoseventhreesixfour8
            8lkqlzghcnqfivefvsj
            eighttwo8five
            cxgj5nineg8one7pcfour
            131
            two35xgxhgnjvb
            gmsdkchjxvv7nkdfczxfjmvhm
            1sevenfkqfour98tsjm
            ninethreefour89six6
            nmqqcnhxeightmnkhsfcjlglkb5
            ninefour4eightqbfivesevenhrxsnxlv
            4sevennine54six
            hsqpjhjtd4rjveightnnvrhfgkzdhxknpqxr
            9tgb37four
            3zqrvfcpm6eightsix
            twopkhshrlksevensjzlfhpfgqkfgrnq7
            frx4drhc7tzxv
            ninetwoninejppfourtxmjtctsn6
            2fbbseventhree9rqtxjnsix1spcnnvmkq
            eightninemfpxcbkzkj1ninelv
            5fsqzdfsxp
            7sixrtrbhzsfsix
            5txbphrdnnine5one61
            one4zvdkkzjbfour2sntphhhzdftgqbtzxfivefive
            5sevenfour6qmrpqfvqlzninefourxzsnthztc4
            x3
            946five88five
            three74cvsxvsxpbrsevenfour3qh
            three9xlhbjmqtwo2one
            eightd4seven2vzgbhc
            six1dvksgllc4eightglpfvqsfx6fourxppsclfth
            tqpjtqgntfourzljvrssls77
            seveneightnineone3
            sixbk5oneone8
            threeeight1sixfour
            ggmn35ksf5dlnine
            six1threesevenmcvspvvz
            8vlqnn7seven
            4sjvkmnvone88
            twozlgjtqpczfour9lgbrsc72one
            59
            lkdkdzlvrdncsixrpgqlxhdrsldxzv4seven4
            3xzq123zztzf
            twoninetnfmqlhxnrfcmvpfqonenrgfvrmf7h
            nsjffflgfnineg3onekzklsxcqqfivelvvzqjh
            6ktvsttwothreethreetktfgnnqlrtdxjjnh6
            onetwofive2two
            56ckkt7
            3lfnhbqrsk2bjnmrh
            zdnh6eightsix4knfkdqbeight
            746ninefive8
            vhpeightwo37fiveseventwoseven2
            8gn5fourvqsvhxsnrbbdpqhmxszdx
            rkrtlbnineseven8tgz4onebtcqzb
            threeseven899
            three4bzqpcjvrjthreekvblvqnszhmhtwo
            2monecfvsix4kbpgbnktdpfknnd
            jksgbgdnvdthree83
            onethreegltwo253two
            1lgxvclgjqjeightsixjhxsrdxzl8
            hptldqz3eightsdftlshlzone
            1jtlqfnk9
            154three
            rlqxzteighttwo6two
            eightfour29nine2mgsdnlsdbeightvbpgjtkzmm
            threejbxvrzrntj311one
            twolscxtkchbdqzlltg9vcqkhbb85
            plkt4jqthreeonethreeeightone
            fp5hjrnqgc1five98ninegdgbh
            vtwovzvdssbone2v
            884kfive7ninethree
            2zjbmdtgbfvjlxbj
            five6three12
            nineoneeight2
            4sixrtzmjb
            kcnnvqcseven25htdxx
            gpvpxvz3961
            cdcsgrjjzsq2hdzrdpzsf3
            pndssrqhr5kcbspqxjxxjf
            jsthk98
            3fiveseveneighttwo2
            87r3
            zfz7nsrbfzgphxfsdgkbfqnlseven92
            npkl7
            onet819seventjfg7three
            four38
            xthffmldlmxxrltxv2eightjzmgngrgpvnine94nine
            dgqsttxqnqljbvl34dcb
            ploneightfour1sixrtwobdrbj1ppmjrdjfs
            68seven9fvzpl4
            8bxnclnsljvbbkz
            htbmxsg7sevenfourvlvmxhhsj
            mpcsfpkthreeninehp1gs6
            3eightseven
            ninebvqjrdsdtkgktmonegktp8
            xcksrtdfksmrlhrjhq823four8f
            lhjsnhgqxxspg6seven
            26mqgmfcone
            twonine9
            4ninefive8five4
            scvkzqqnr24ninekmvh47
            8six56nineeight
            mr8fourdl9seven
            hfgrhgfhksdgrpgzk9vljzthree3cjkz2mxrq
            fourtfhlzjrtl53nshcqrjv8one33
            five5n87dhncbkveight
            lx49threeonetqtsj
            seven29eightfivetwo4six
            9xzqsvxbj9
            nine4674zxrctgqtk
            1two4
            6eight3jghjrqzv
            53lksnfcftgcgmfthreehxkeight
            3sevenfjvjffpxqfivetljmbktvk9twojcl
            six1lpvtsrq9zqcqlgtdbdzxrjk
            418tfk4hghnxbjpk
            7fivedvtvbtmvv5ptwozfgq7
            threeplkvctrgr43gjnlfour2bzh
            bcfjntzmntwofiverbhsixgqvmpkj3gmclxtljj
            764jbvbksgckc42vhqpfmb4
            qsq922njbtnqpsqkt
            68388qp9kgmmqcv
            poneightfourfivefourxfive7nine
            6onethree6two86four
            sevensixk45fourthreetwo
            4eightfdqspzsnvrnztg
            64mhll2four8
            gmdmgzcqqnghmrgdkhz1vhtwoqspfkzhctvgdg7
            bbheight7xlxbmvonesix5
            6rcfivejdvffnkd
            vbclrqfj11hthrczhxdjthree9
            dvgk1nszpptjsz9knzbbtbtp
            gsqpnb5nineklhn6hnrbseighteightslxs
            nbnqlseven9vmlbfzv
            threelnxztmlmptwopsmnntf89
            3kccrnfour
            3threeeight
            tccxcs9five37one
            xmzpjth2frrqnbc
            fivedmtplmsixeightllqbsdps941eight
            8rdtsix
            2threem1four9
            eightrvgxlrlsixeight22
            onefivejqvmdsck2sevensix7z
            fourbxgtpqdfonenzpqxthtchxjhzlqnk24xlqtjrtn
            eightshttzglmvdscttkzone74sixvkvmkdcq
            kvphvtjcbjdsblbnvgxxninectcrqzsqgstwonineltrlldpvk4
            3threexmnb
            fcfsttx4lnjr8twonef
            sevensix2fourgpsx9nine
            621hxrzxqlnnsxlb8jgj8three
            one9ktfzq
            mzk1onenine
            779five5pc
            43sqfknineninetgscnbmnxvtqdk
            cgb3fmfmpgqfzstfkm
            gktbmhp25eight1hbrhlzplkk
            12nine6onetwoftjrjfrnxfng
            fivekk1pvhhzzrvpsxtphgtwoeight4
            mdqjkbnvtddldkbtbcvrgbnhllgtwothreetrzrn8
            vbsbfvgrq5twoseven65nm
            mgbseven9fournine
            7sevenone3
            ttnonethp4
            zpzxktxbstxvkkqjbmrzqvm4txsfhcmlr
            pjsix48fournksgvfgkvjqvktnh7
            vbbdxd2fivetwo
            four2lmsixxgckxkbsix6
            fourxb4
            rphsevenxm12mstbdvtjx
            8szlthkxxnb3two55
            zgjztbtxmgqzvhhnljjhtwo366335
            2eightfourfourtwo958oneightcps
            two37kfkfive
            jzctwonekbtqdnzblonefive6gqssflbprspdzmfzhbvlg
            one8fivemmgnine1
            9sevenmmfrjs2three4xfxcnine
            5chxgkpnsvmmvbgpsvrmfvh
            flfbssqninenineeightzkcmvseven49
            8six44qtstjsm7nine
            sixgtgjzbhb5seventhreeseveneightfqdqpvtone
            seven3nine
            553fqnvr
            one6sixg5twofourthree
            threesix25
            kkgggk4threehbhk
            1fivezhmhckfmbfzkxx5six6
            6jjzjmmbddn365dpcgrrtcbkhpq7
            1xdnxgtwofivesixonenine
            9bbbkthgghpmvjbpgnjfour
            five2sixljg9gcjfour
            zsqs8nhtgg7fourfive
            4zlcjrtkpmsevennzjxfourdtwo
            fourmdcnhz1vqxqh
            hxknxpqdhrgqt3fivethreekmfm9
            sixsixvkczqlghfhvzclnxnxpthreenine2twonerc
            2vgqs3twoktx66
            z6fourmlmmmmtczl
            mhskb47
            eightsixnsk4threepdsxdsqmcp
            vlsmnznscsix6sixdjhkdjprgkfive
            zjmxfzpst153six2
            8ninelxfourtwo8glkxrvqphqlcsqxjvjmgf
            31four
            hdmpgjrkdthree1two
            5eightfour75jmbs
            one9twoztjpvxqrgrvnt
            pgvpsqnmnnine36pfp
            1pmkhmqrlsbtthreetbxhpsp
            2761xfgtwo
            1kqtnqjmx5
            5bcjgmrpvbpmktxnvd4533qh
            67seven2
            hrseventwoqxbnggjf3gszbxxprkbjgdgjgskxclphvcrt
            24twopjtcvnvhmdj
            two1ninevphz
            zljmbl4threetxqlgz
            four69threethreenine6
            sixvhcxbnsone1ninesevenj
            eighttwo67
            27btwosix
            hglnkvmzp6sevendfqxbdnnhf
            tjhgnpbq3four
            1six68four
            796mhqxm7
            seventclmhcvmljpbxvvtpcm8mnfnone22
            kqrtwone6vhlshrvx1
            5djgneightfivejfmvgmssscx
            71eightsk93
            7tjxrnine63four5
            5ninechr
            jgdjcxp2sgdpkvjsx696
            nineonetwokpvrxlbsevenfour145
            tpdfcqklninepqpdnzckxq4
            fourfourthreeqvlvvfourfiveeight7
            5fgkpgv97gcfslvms5
            xp7ninetwo3eighteight68
            five6pgncxqdcdr2sixseven
            shzm9gqgpfpkhxnine
            nrzhmcpbdhvkgftqtseven5fourfive82
            6threenineffrdsqtblfcfour4
            vvxfk438krcn4seven
            ninethreeseven1
            seventwosevensixpblbldrfive1nine
            8mpzjhzvjdj5zljjconeseven6gsz
            fjzcmctxjxqrfdls9kkcdh594
            pgc1n
            shheightwo7eightfourczdvnzhxztphrnj3
            155
            sixmdlbdxs1two7nineone
            onegvmnsqvjbgcxxnine65pnvfivesix
            onemfbznqjsgfive9threefdmb8gkc
            2three1
            4sxtrr
            brzkseventhree51khpzssjczseven4
            5xtkzzrqgjzthreeone
            fourfivez48
            4vlcksgvddfvqnjctwo
            fourhmrd2six1vcrgqsgnndn
            dmcklfkpt63two8
            5onebqbjts2223lzftpxdhrs
            three6mtvkhsone7fiveghxgxz
            sixfiveone144seventhree
            9kqqjmtkrb
            93qsrktlmvtksix7
            cknsdxjkninehftwo7five
            twosix4eightsevenonethreefcrpcp3
            three1oneeighteightbhlrlgsx
            twosevenone1
            rdgcvvlth1fiveone9
            ksbkbnthree4rvtpftfctvmcfive
            vnkprjzqtmsbhglqfcbfd3
            hfrqcjdcztxntvjrpjf7fivefive875
            4128plkhjjpgnxeight
            6ffxpgg5threenzg628nine
            3six3
            seven4rbzxgvrktq4mrrcvhgrkhsxmgtkzslzhzsrn
            tfzhdgqpfoursix4zqqeightzmjhrrfourptcfxt
            seven7sixfvnbnzbfive1zzqvnmxzjmnk8
            qneightwo6fourninefourseven7five
            3dhqlct8fdcxxhp6hnb
            six6eightthree1xdrtzk
            7two1
            d87fourkrqzjfs
            xglckx1two3jtnmmq1ftpmr
            1sixtwo4sfourvprscvzf8
            36fiveonenineone
            ncjblr43bljpgcbxpeightseven
            9onetqhqfj134ftmd2dm
            xjtwonepbqhkzbztrlsktkfxspbdj59bjzj17
            7nine6sktjpsxbxdq26qqqprk
            twofoureight1two
            fkvsfiveniner3
            nvsmcrpseven242threeqjlrs
            76xrll28qhdqltt
            8sixkkxrtwon9hm
            5jnnqnsvfxcptrktmbpneight1
            xvcxsrdhzeightqfvh9
            3cdtz32six5bdskvjrksxjone
            gbddhxbm44bsl
            jxkmntkx1eightwodp
            three27jlmrldnsj3
            6sixjmlgldbx61six3sixone
            three2pjvbrjkdone5
            8oneightgp
            nine4seven5
            tr4mmtcgvxvqfgqd6seven625
            twofive8
            26fivefdfkgtvsvvxlgrrjlxxseven
            ninethreetwo34sevenfivevxzshkpzxk
            jm1fbtptbkjfx529
            sixjrl98threetpxcnfive
            6rnv
            fourtwotwo4qhh4
            eightdldkblq1six
            442two
            ssix7fivefive
            four4eighteightmchbppzmm
            fivek8dmjsbtjvpj
            five6861vzrqhbfdvsixmqsl
            ctqrpzgpgcqhtwo8twoctb9five
            eight4threeeight7
            one359
            pxhhkhccd6vvbnbfourgjmhzvcs98
            65qskzgm7seveng
            9dcmtl8sixj6
            gtbvnth1three3zfmdfdnine9
            hpxjjcj6lkkzdv3xczkpbprcnvkfngv
            foursixseven9
            xnlf6ccxzlcsixjvhkxzxfck
            fivembqp369lzgrfzq
            sixfourninefivesevenqgdfbmhthf6
            fxbtjfkqsmvnsrvfrbsixvzmrzdtvvxhbd5
            kbsxlq97
            52sfsqq4onesix
            8leightsixtwo
            hvrkcdbdxz9sixkgpp
            four7three
            9t6hbsjkznxchjq9zbdvkhx
            7twoeightnt
            4qmqlvlpk6
            xfqhf74eight
            xhtpninecfpzjq74
            kfvdfkqtwoksbknptnrcqlghbfive4fiveftnnkv
            vd1fourlfsfxqp
            cnxpjjbklvlbrzkqgfmrkeightgbkgscbgpsix4
            sevenczftbcg24rrqdscm1nvtddt
            sftwone1fourtwo42nine
            2nineoneqxtmbsgrqvfivecsstrbstptwo
            three2seven7qlcpqqgqdhjxbvf
            kdkbmnbsxpgtpgzmtlbftmrfmcvmdghvhkxl4
            two6nine6prlzxrclq
            qtwonefqchmsbtgk876three3
            6ltnntlnvtfive
            4six46pcjgclkstwo6zspgq6
            341three8six
            four9onetwo166rvcgtv
            gbhgsnkz6eightfour6sldckhdnbg6xktdfb
            one9three67
            hhvcphgxrzfjggthreeseven624
            three374tlfl2nine
            vkpj3
            three1dglzmjvps57tworrqglxb
            sixsix18six
            nine8one1dmgmkchvpvxqpdqdpxzxpthree
            mpsdqgmtllqhptwoninevrkp43
            3sixddcklgnrrj
            gpskjbbchfive58six
            72v8seven
            4sxjvbd
            dlqs87seven7zphphqfive
            341mmsfdff
            1lftv2
            five1three4xpt993
            92nhcncp
            58eighteightgjndsmsjtpnlleighteight
            9tnrsnzqzkbkseven
            ffhhnvfbmsjg2six
            4xtvqpsjnine4two
            57ddkkkltv
            nine8three
            rvkmcmshxpkrxpglzt3khcfour
            eightonebjqbljgmt6tfkrf73
            three36jtgml1
            fndcqgm2fournngpvlcmcrzrvsevenseven2
            f9ljpjpnkseven
            cxhtfspj9mkfxdfour99
            tkgjdj4twos2four1
            5threezrccdbf6qpzxvdvmlrbmkkpzpxdrtwosix
            eightrstwoxkbxvmdjgxjftfzmngsix6
            66onebrtfdpzqlone69four
            three2five8hcncdsvqfx86
            7fivefbone
            twovbqzfhkxgv5g7
            5lfprlrznvgtsix4xmdxvgsv7six
            threethree57
            five8threetprlzmrx418kdlzddbpv
            three7hqsnvrxhv68five1three7
            2sixcj651three
            cvfqbfftpk3six6vdvx6eightxqsjqph3
            914
            2zq9
            kzfsevenninemjpfourhqq4jfgqvdjeight
            rvgmtnjsix9hvncrblxfour176
            threeeight388three87
            twovtgshgfive7
            qseven4eight1fivejcrt
            oneone7
            6fkfddtbpfbp
            5six7three
            1fourfiveljrmbmfpsvzzhdlh
            ndqhpmjsxjgvdgv2six1qbnc
            sixeightsvcgjzfmqthree98
            threeglsevenone2
            9onetwovlmzvndpts3
            six6xjfnrqbmxs7twofhcsgnm1
            42pspfbrfive
            threeccbdtsrfv4drmvqcbdsix7sevenfiven
            7rvgchnjfznc
            sevensevenkpbggfhrhk121
            2hzvqxxtrmlxbvbclkfiveone
            9fivepdpmmlkshq8kqlninefour1oneightt
            5sixeightsix37
            four9sixqjbpxfgc68zcjffmjone
            qkpbblqtk6fourvgvdvnsdhnktgqzhxrm
            qbcvmnpxv1fivexlone4eightjlqdvbqmr
            vgt35pztrjktfzz6
            tgfmjb7eight1ln8lhbpdcz
            4xk56threethreeone
            258kzkzrpsclmmf
            dvrfk9fourtwollnsgcn
            tnmphhv4
            sixtvtdptlpseven18dmrnrfgdzgcfrlczthree
            llscxn2
            487eightfour73
            7skmgfpgdt69two
            pbkprbzvs819threeonekjpk7brkmbqbkgroneightb
            ninerfcvgpbltbljnk3
            nfzlonesmeight6gtff
            tleighttdxtbhrvgk16bpkmtcvlnrhnmhz
            one99
            mqtwooneeight7sevenfourht
            stzmqplr8gvmxblz
            five8dvdjqfmpnh3
            scjjr4twoh
            nine35gzmlt
            5sixfour2qxsqkpnq
            kdkjqdkvgs2
        "#;
        let result = trebuchet(input);
        assert_eq!(result, 53268);
    }
}
