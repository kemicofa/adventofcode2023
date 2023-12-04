/*
    --- Day 3: Gear Ratios ---
    You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
    It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
    "Aaah!"
    You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
    The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
    The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

    Here is an example engine schematic:

    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..

    In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
    Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

    --- Part Two ---
    The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

    You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

    Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

    The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

    This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

    Consider the same engine schematic again:

    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

    What is the sum of all of the gear ratios in your engine schematic?
*/

use std::collections::HashMap;

use utils::split_and_clean_input_into_lines;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    split_and_clean_input_into_lines(input)
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

impl PartNumber {
    pub fn is_adjacent(&self, x: &usize, y: &usize) -> bool {
        let x_offset = x + 1;
        let y_offset = y + 1;
        x_offset >= self.x_start
            && x_offset <= self.x_end + 2
            && y_offset >= self.y
            && y_offset <= self.y + 2
    }
}

fn build_hash_key(x: &usize, y: &usize) -> usize {
    x * 7793 + y * 7019
}

pub fn gear_ratios(input: &str) -> u32 {
    let rows = parse_input(input);

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols_positions: Vec<(usize, usize)> = Vec::new();

    for i in 0..rows.len() {
        let row = rows.get(i).unwrap();
        let mut start_index = Option::None;
        let mut current_gear_number = 0;

        for j in 0..row.len() {
            let cell = row.get(j).unwrap();
            if cell.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(j);
                }
                let digit = cell.to_digit(10).unwrap();
                current_gear_number = current_gear_number * 10 + digit;
                continue;
            }

            if *cell == '*' {
                symbols_positions.push((j, i));
            }

            if start_index.is_some() {
                part_numbers.push(PartNumber {
                    value: current_gear_number,
                    x_start: start_index.unwrap(),
                    x_end: j - 1,
                    y: i,
                });
                start_index = Option::None;
                current_gear_number = 0;
            }
        }

        if start_index.is_some() {
            part_numbers.push(PartNumber {
                value: current_gear_number,
                x_start: start_index.unwrap(),
                x_end: row.len() - 1,
                y: i,
            });
        }
    }

    let mut map: HashMap<usize, (u32, u32)> = HashMap::new();

    for part_number in part_numbers {
        for (x, y) in &symbols_positions {
            if part_number.is_adjacent(x, y) {
                let key = build_hash_key(x, y);
                if map.contains_key(&key) {
                    let (count, val) = map.get(&key).unwrap();
                    map.insert(key, (count + 1, val * part_number.value));
                } else {
                    map.insert(key, (1, part_number.value));
                }
            }
        }
    }

    let sum = map.iter().fold(0, |acc, (_, (count, value))| {
        if *count > 1 {
            return acc + value;
        }
        acc
    });

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(gear_ratios(input), 467835);
    }

    #[test]
    fn it_works_with_puzzle() {
        let input = r#"
            ....937..........309.............191..............745.................913......................................256................891.......
            ................*..................-..113.......3*..............219.........495%................40....97.......*.......670.60..../..........
            ......919..928..511......................................%......#...&...................................*.................*.................
            ........*.....*.....605..990....765.....&..120...159.....691.........1.....67*84.92..........687.580.....56.408..677.........$.........624..
            25.......911.525...........#......*...985..........-.&...........645.............*.......408....*..................=...546..994.......*.....
            ......................340*....&..821......504........60.11...........*967......114..........*......545...................*........=....518..
            .548....810.....898........934...........-.......808....*.......323....................40..260........@.......827.......27.....903..........
            ....*....*......../..................11.........*........691.....*..450..........#....................................................564...
            791.546..944.848......588.138.................875..772........587.....*.98.$....941....*638..........534*271.../..........73...56...........
            .....................*................................*.............787.#..173......139.......&..360............368...838*.......*547.......
            .658..358......985.257..@..645....636........542....24..........................+.............99.-...502....................371.............
            .........*.............897..*.....*....628..*.............................#....13..853.442.............*.............586*......*............
            ....619.4.......635..........827.......&....217...........164..........@.205........*...*..............229...............656....243....538..
            ...........603.*......./..........167...........96..615....*.........512.........494..689....$696...............423.................35.*....
            .......625*.....221..424.............$....=......@.....*..167...........................................127........*591.......505...*.......
            823......................804......*.......987..*.....819..........754.970*541................%......584*......911............/......559.....
            ...=.....423*297.....400....*..999.781.........472...........................................618..........+..*.........=.......#152......315
            .....52.............@....320.............................922........977*899.............................646..162.544..441...................
            ......*...542*54...............=.........698.229*836.......$......................-..........752..................*........*.......238......
            ....608.............-.748.......524...-...*............241........911.813.529.....618..309.....*.................730..@.....8.........*423..
            ..........%689...514..*.....255.......28...........665*.....+......*.....*...................539....146+...974.........432.......546........
            680..*77.............391...................994............950.......58......59......700..................*..*...727-...............*..585...
            ................980.......858.........+..........820*351........./........../.........*..................54..69...........*......949........
            196..132.......@.............*...../...481.-822...............535....378...............203.......541.............644-..111.890.......@......
            ....*.....926........%413.686......512...............722.............*.......................231....*.648*940........................747....
            ....872..........749............/.............930...*.....498.....784.....892...................*.190.........625.=90..115...............277
            ........236..604*......890*981..401............*....981...*........................635*634...901..............*.........................*...
            .......*...............................310......552.....893..........146...930.............................44.635.129...54...........751....
            .......6...41.307...763.......549.........*....................#23........%.........927*......774+.....=...*........*...........335.........
            ...........*.....$...@...984.*....590......452..........%..........39...................960...........765..730....859.&....242..............
            ...557.....129............*..796.*......3.............405................422....+...........61&..178..................539..*....786.........
            ...%............./.......330...............%.193............246......663...$...694.....836..............610.................848./...129.....
            ..........239....219..............*824...762..*................*.....*...................$..254......................796$..............*....
            ......440.............623......781................164........445......529.+.......692..........*............................517.........80..
            ........*....538.....................................*231.................34.798...........942.960...492...........579...$........26........
            .........609....*...758......*....-............@.789.......+52...116................153...=.............*...975...*....709..899..*..........
            ..319.........595..&..........700.497.......826....*.............................../.........535......959....*...288..........*..280........
            .......797.............................105......*5.......402*249..50...@..979.846.......460.................302............537..............
            ........*................=................%..449...720.............*.486..%......*.........*.624...209.100.........227.................792..
            .........512............448............@.............&...816..................756...825..379..*...*...................*442.....465......@...
            .....752...............................494..................*....474@.....575.........*.......83..338............................*...=......
            .....*...144.258..373...&......997.............431....874...382................295...422....*............67.......$.............895..40.394.
            .....287....*.....-......296....$......................*................./................676.......254%..*....707......131*704.............
            .226.................562........../.............#....751.....268......108.....380..#.....................685........32......................
            ....=.789........684..*............910.58....232.........786.....723......121.../.23.......968*970.446.=..............-.......301...........
            .........*425......@..20......712.................701...*...............................................165....475.....................61...
            .209$...........................*......644./493.....*.94...#............144..260..69......134....................*...751.........981....*...
            ......788.......#614.....904..292...................1....377.............@........*.......*......../.......919..948.....*..513...*......796.
            ......=.................&.............941*.............$.........14........85.434..556...572....745....282...*.......621......+...709.......
            ..............364...........................377+....254...............111/..&....*.....................*......403.........339...............
            ..531+....666..%..148.......39.905*459..505.......................................991.159..............660.........=......*......8.936*.....
            .............*...*.....685../............*..........-497.....805*317.....853+...............332...............989.577.....558....*..........
            ........204.667..629......=.............114...............%.....................@...........*.......&...........................632.........
            ......#.....................929..................978...349..........771......764.........875.....274.......316........*.......%.......*709..
            ..807.960...........894....-..................39................979....*....................................*....883..916...378.............
            ...*........#................633................*977.............*..990.........205...................438...429....................956#.....
            .646...%....2.........$...............229%.265..............84...................=......746.......443*..............688*339.................
            ......63........86+.75........593.............*215...........*........478...&.............=..............27.152...%....................331..
            ........................985....*...................932....640.............218..................676*......#..*....710..+121........227.%.....
            ................................884.475.........................118*645.@.........30.400*.125......290......698...............910*..........
            ...725.........401.......990.........*.....525...389.....................475..............*............*322......933.....14..............719
            ...*......772.....*......*.......261..99...&.....*........96.......*...............594....429.......870..........*.........*................
            ..198..-..*.......873.396..........*..........726...........*70..775.........690...%..........83/........%...274.829........156...227.......
            ......386.99..............@....53.....104.............................738.......+....................164..83............476.......%.........
            ....................../....353..=.457*....275..24....................*......384....579..............$............182.......*416......550....
            ................-.....937....................*.*..........884.....401..961..+......*........................21...#......*............*......
            ....462......521..........#....532*...+125.771.401..........+...........+........619...=................955*.........630.151.506....343.....
            ........................242.....................................696..................786..881.........=............-.........*..............
            ...652...=.%26...................967.........@..............331*....746........./............*.....126...+...=...71.....+....975............
            .....%.726............48.493......*......@.239.......212.........../.............110....$551.............545.121.....45..436..........=.....
            .........................*.......489..780...............$............................47.........315...................*..............412....
            665.................425...930...................649.581.....=..385.............975...*.....103..*......217.443......763.....................
            .....-................*..........615........703..*.........532...+..857....542..=..989....*..............%......................542.........
            .....279.......621....481./786..#..............*..75.@................#...*..............222.......30.......728......&495.=905.......124.206
            ..........509...*..........................-..........817..................34...593..................*........*..937........................
            .148.........*.772...........539*199........73.............733.................*.................23.298.....596....*...................141..
            ...@......=.............331.........................589...*.....70...........72............402...#..............948....790........89/.*.....
            ..........578..338........................161..............352....*.....464..................+...........257..............*512........808...
            ...........................103.............................................*.899*580........................*.....................595.......
            .....*776.....700.............#...580.....521*845.......29*67.............37.........-178.......=...........970.......569..686*.............
            ..997........*.....656....351........*662............30............392.......272.............926.......................*.......933..........
            ......458...544.....*.....-....137+............83...%.............@..........-.....................268.756........=.640...............526...
            .....#...............5...............453..752.........................................#600..........*......172..997...........539*..........
            .........355$....79...........376...*...................424...337.166..927...711....................920..............+818.........173....884
            .71*53...........*............&....938........599.18...*.........*...............187=...........599...........................496.......#...
            .........../....997..........................*....-..237.....303....256...807.............#393.*....454...............*397......#..697......
            .....@.....576.......748.184........650.....429..........220*.........*....*...................15...*.....$...146..831.....*................
            ..976../................../........@................@..................154..598..........328.............799....*.......755.................
            ........23.......564=..........771...631*.....893.540.......................................*...597...........458..................%....871.
            ...226...........................#.......612....+...............57&...&.............#12..115...@......967..............78..823.....763......
            .........777.-.......211.886.421.........................42.........799....284.99.................#......*..238#.......#...*................
            ..........*...564..../..........$.&889...649...............=..................*...585.265..........855..781........-........865.............
            ....433..90...............................+.........456...........................*.....=.......................175....*482.........332.....
            .......*.....................306.....*.............-.................@..........212...........879...@...............725......552...#........
            ........2.......529......914*......256...........................999..840....................%.....5...........43....................882....
            ......*.........@......................40.653.........681...................984.580...944...................=...........-884.......&..*.....
            ...959.76.#867.....419.............*.....*.............*.....765&.............%..........*......516........410...............481.463..71....
            ...............501*........895..178.433..........291...137......................#.........658....*....672*.....9........216..*..............
            .........478...............*..............660......................748.590*193...851.85........939........320............*...227..+.........
            806......*......548.......309............*......$....91..918..........................*.............................758/.524.......653......
            .........588.......*58........494.....291....97.231................932.......398.......457............905...................................
            ................&................#............*........942............-..267...............214...........*..931..........816....$...........
            ........&......5..614.=180..................60..........*...#...308.....*.....397......67....*........805..................=...178.542&.....
            ..671...574.........*........+.......559..........551..370..322....*.210.........=..51...*..803............../.........$....................
            .....*.......507..360.........679...=.....421*378.+..............51...................&.344..........676......606...564......360....+.839...
            ....935.......*.......159..................................530-.......480...112.................954../....214.............../.....981..*....
            ........932..512..143*....................=.374...%249..................*.....$.352...............*...........124.11*380....................
            ..........*............482..............963....*.......363.....*557......34......%................499...../...................%.............
            ....368...174......536....%......#..............513......*..847......................*575.485..............578..............1.625......556..
            .......*..............%.......5.794.......401.........877........26....*..........195.......%.....................316.381..........419*.....
            ......437.......583.........../.......214.*......................*......433....................@797.......174*524...+.........989...........
            ...................*778.&643.........*....187..#.....%...@.......86.........178.......629.............................................*.....
            .....................................822........691.557.1..................*..........@...................204-.618.205.............418.631..
            .....659.............@.................................................892..811...689.......56....................*.....@.611...............
            .....*..........*876.780.43..567.........325....437..896.........@........*........*.........*........................824..*..6.............
            ...159.......672..........*.&.......599.....*....+...*.....$...210......232........47....................*586..794........90......212.......
            ..................427.......................696......131.470................507.................696....95.........*...927.........*.........
            .............925$.............887.....799.......332...................395..*......................*.........261-..441..........691......857.
            ....834..................728....*........#.%212....%....41*...681......*...881....604.......%..977.........................185.........-....
            ..................../..2*......401...............................*...492........&....*829..928..............................*.....*256......
            ...716.46=.........217....897......975.........561.......839...38................441........................................................
            .............=.............*.........*...........*......#..........&......211@.......475..........766..........................357*567......
            .500.........81..../......915............=......857..#....102...409.....................*301...15.-....725.........608..213.................
            ...*................192........736....877.............529...............994.....................*.......*.....399...*.....*.................
            298...734...827............500*...........................802......819..........................448..257......*....240....264....527*285....
            ........#.....*.....256*..............643.........957.70..*......3..........276...........677.................753...........................
            .............634........765...741....../...-..344.....*...512....*............*.......499*.............283..............*....157............
            .....887............150...................217..-..968..94........376.......812....1.@......142@...-....*....277..720.115.210....=...692.....
            ....*......464........#...........571............*...........750.....128*........*..724...........325.71...*.......*...............-.....392
            .....679....*....378......+355.69....+....428...844......$......*487.....395..278............*335........62.....335......724..704...........
            ..........907......=..55*....................*.........795................................397..............................*..$.....$...#...
            560......................753.........*....260...............................149*656.................526......426..390.....98......286.519...
            ...$................*123..........803.749........572.........642..620...696..............$...........=.........&.-..........................
            ...........646...331.......................245......*...637+.*......*...%....606..........707................#..............................
            ............*.................$...............=..998..........391..955......*.................582.....*822...148....%....388.....406..893...
            ........770..901...183.260.836..594........................................653...................*.239.............286..%...........$.*.....
            ....710*..............*.................709..948..............504$..............624.......%...214..............................303.....752..
            548...............642...393*469.@.......$....*......................172.........@......860..........170......15.............-.....*.........
            ...@...........*.....*...........155..........998.........657.851-.....*...............................*.....&.........972.751.249..........
            ......898...561.186...207....270.....................................968...231..181..................324.........696........................
        "#;
        assert_eq!(gear_ratios(input), 84051670);
    }
}
