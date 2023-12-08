/*
    --- Day 8: Haunted Wasteland ---
    --- Part Two ---
    The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

    What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

    After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

    For example:

    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

    Step 0: You are at 11A and 22A.
    Step 1: You choose all of the left paths, leading you to 11B and 22B.
    Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    Step 5: You choose all of the left paths, leading you to 11B and 22C.
    Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
    So, in this example, you end up entirely on nodes that end in Z after 6 steps.

    Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
*/

use std::collections::HashMap;

use utils::split_and_clean_input_into_lines;

#[derive(Debug)]
enum Instruction {
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct Instructions {
    values: Vec<Instruction>,
    index: usize,
}

impl Instructions {
    pub fn new(values: Vec<Instruction>) -> Self {
        Self {
            values,
            index: 0,
        }
    }

    fn get_instruction(&mut self) -> &Instruction {
        self.values.get(self.index).unwrap()
    }

    // indicate if moving forward
    // reset the index
    fn go_forward(&mut self) -> bool {
        self.index = (self.index + 1) % self.values.len();

        self.index == 0
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn new(left: String, right: String) -> Self {
        Self {
            left,
            right
        }
    }

    pub fn get_destination(&self, instruction: &Instruction) -> &String {
        match instruction {
            Instruction::LEFT => &self.left,
            Instruction::RIGHT => &self.right
        }
    }
}

type Network<'a> = HashMap<&'a str, Node>;

fn parse_input(input: &str) -> (Network, Instructions) {
    let lines = split_and_clean_input_into_lines(input);
    let mut lines_iter = lines.iter();

    let instructions = lines_iter.next().unwrap().chars()
        .map(|val| match val { 'R' => Instruction::RIGHT, 'L' => Instruction::LEFT, _ => panic!("Should either be L or R instructions")})
        .collect::<Vec<Instruction>>();

    // ignore empty line
    lines_iter.next();

    let mut network: Network = HashMap::new();
    
    for line in lines_iter {
        let (key, raw_destinations) = line.split_once(" = ").unwrap();
        let (destination_left, destination_right) = raw_destinations
            .trim_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .split_once(",")
            .unwrap();

        network.insert(key.trim(), Node::new(destination_left.trim().to_string(), destination_right.trim().to_string()));
    }

    (network, Instructions::new(instructions))
}

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_numbers(numbers: Vec<u32>) -> u64 {
    numbers.iter().fold(1, |acc, &x| lcm(acc, x as u64))
}

pub fn solve(input: &str) -> u64 {
    let (network, mut instructions) = parse_input(input);

    let starting_node_ids: Vec<&str> = network
        .keys()
        .map(|c| *c)
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<&str>>();

    let mut node_keys = starting_node_ids.iter().map(|v| Some(*v)).collect::<Vec<Option<&str>>>();

    let mut steps = 0;
    let mut steps_completed_list: Vec<u32> = vec![];
    loop {
        let mut next_node_keys: Vec<Option<&str>> = vec![];
        steps += 1;
        for i in 0..node_keys.len() {
            let node_key = node_keys.get(i).unwrap();

            if node_key.is_none() {
                next_node_keys.push(None);
                continue;
            }

            let node_key = node_keys.get(i).unwrap();
            let instruction = instructions.get_instruction();
            let current_node = network.get(node_key.unwrap()).unwrap();
            let next_node_id = current_node.get_destination(instruction);

            if next_node_id.ends_with('Z') {
                steps_completed_list.push(steps);
                next_node_keys.push(None);
            } else {
                next_node_keys.push(Some(&next_node_id));
            }
        }

        instructions.go_forward();

        if steps_completed_list.len() == starting_node_ids.len() {
            break;
        }

        node_keys = next_node_keys;
    }

    lcm_of_numbers(steps_completed_list)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "#;
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn it_works_with_puzzle() {
        let input = r#"
            LRRRLRRRLRRLRLRRLRLRRLRRLRLLRRRLRLRLRRRLRRRLRLRLRLLRRLLRRLRRRLLRLRRRLRLRLRRRLLRLRRLRRRLRLRRRLLRLRRLRRRLRRLRRLRLRRLRRRLRLRRRLRRLLRRLRRLRLRRRLRRLRRRLRRRLRLRRLRLRRRLRLRRLRRLRRRLRRRLRRRLLRRLRRRLRLRLRLRRRLRLRLRRLRRRLRRRLRRLRRLLRLRRLLRLRRLRRLLRLLRRRLLRRLLRRLRRLRLRLRRRLLRRLRRRR

            DBQ = (RTP, NBX)
            NFX = (PXX, PLG)
            VBK = (BRV, DKG)
            BRS = (HLR, VBX)
            DDK = (SPR, TCR)
            FTS = (LJB, MDJ)
            BLH = (DFM, GGG)
            PCC = (GQR, RHD)
            QKN = (VVR, GBL)
            KHN = (FNB, LLT)
            HTD = (NPJ, BTL)
            FPL = (BRX, XQD)
            RCJ = (QKN, XPD)
            XRN = (RMQ, LQB)
            HGM = (VPV, SVR)
            RFG = (TLM, KPH)
            SSG = (QGC, KJV)
            LPA = (QQN, GNF)
            SVN = (VLM, BDX)
            VPF = (TTR, RNQ)
            VNK = (DSX, MCZ)
            KMS = (XNN, MTB)
            RLS = (NHX, CRT)
            VRS = (VBX, HLR)
            TVK = (MKG, NKB)
            CKH = (DFF, XPV)
            KBS = (LJC, PRS)
            JMT = (FPT, DLX)
            LRV = (QCD, RLF)
            TMM = (VPF, GKD)
            HKH = (PRS, LJC)
            GSM = (SVH, KRP)
            SLG = (PTL, SNJ)
            GVS = (DSN, CKH)
            QGC = (HKS, DFP)
            RFT = (VVQ, GMF)
            FDF = (JKV, JKV)
            MRL = (PKS, TKS)
            HBN = (BDX, VLM)
            CFB = (PPV, QVS)
            TFJ = (KGN, JND)
            HLB = (PFV, JXR)
            RFJ = (SKL, QCN)
            PQH = (KGN, JND)
            HGR = (FKL, XHR)
            CTB = (HNQ, HNQ)
            XMN = (CJV, LKQ)
            JVX = (FSS, CRC)
            SFH = (MXD, RHN)
            BRK = (QPL, DJB)
            HBX = (DSN, CKH)
            RFL = (VXB, FNF)
            GXP = (LHR, SMV)
            CHK = (NLQ, VCR)
            QPG = (HHL, KHB)
            VBC = (NFV, FST)
            QJF = (JXL, JJC)
            XKQ = (PCS, BSH)
            SKS = (SNN, QSN)
            GJJ = (VVG, XPR)
            DJP = (SJM, DNR)
            DQF = (MHG, BKZ)
            MSB = (BRX, XQD)
            PQR = (TLL, RVS)
            XGB = (RMT, HGF)
            HBD = (PFD, FDN)
            NSN = (RFJ, NJT)
            SNJ = (MVG, DBQ)
            NMV = (SKG, TDJ)
            PTN = (TNR, SSG)
            SBG = (CVH, LMJ)
            RDT = (SVQ, JTT)
            MRC = (BKF, QMJ)
            JKV = (CTB, CTB)
            CTX = (QHL, BDV)
            SQM = (DNR, SJM)
            MVG = (RTP, NBX)
            KKJ = (TQP, PST)
            QKX = (BRD, SFT)
            VFV = (NTL, VMQ)
            LXK = (SSL, MFR)
            HDH = (VPS, MJH)
            RTP = (CGJ, XJC)
            BXQ = (FLN, FLN)
            GKD = (TTR, RNQ)
            GPP = (SNN, QSN)
            GDQ = (JLS, TND)
            LTF = (GXP, TVB)
            GVB = (LSV, FRL)
            JCF = (HGR, GMX)
            VCR = (JTR, MCM)
            XJC = (DGP, XML)
            XKA = (SFC, CHQ)
            LJL = (VPX, PMS)
            CPH = (PCQ, QKX)
            VPV = (TFP, NJN)
            TDJ = (PRV, DBK)
            JGQ = (TPX, NMV)
            LJS = (GDQ, QPV)
            VHN = (CBJ, PFQ)
            JHT = (XHM, NRK)
            ZZZ = (HJS, LRV)
            KJV = (HKS, DFP)
            HNT = (XXT, KJT)
            RHD = (KQH, TMD)
            JKG = (HXJ, MGF)
            SPL = (TTB, STB)
            MBD = (CQK, NHH)
            TPX = (TDJ, SKG)
            HRC = (LBB, JLR)
            QHN = (JHT, THC)
            SPM = (DDG, MBK)
            MFG = (JBD, GSM)
            MHR = (PTN, HVB)
            RLF = (GMG, TDQ)
            XBQ = (KKN, QTN)
            GLB = (DHS, SXC)
            PRV = (PGC, NSK)
            SJD = (BXQ, GHF)
            THC = (XHM, NRK)
            GBL = (GGR, HTK)
            MDK = (JXX, DBD)
            NJT = (QCN, SKL)
            LLG = (KBL, LXK)
            RVS = (DDB, QRV)
            LHR = (LPL, TGJ)
            TBD = (NNB, CQH)
            DSN = (DFF, XPV)
            QQN = (HRV, PTT)
            XPP = (GSJ, RDT)
            CBJ = (JQL, RBX)
            RBX = (CNV, NKQ)
            DDG = (NBS, KTV)
            XNN = (TMC, GCG)
            DSX = (CGR, SGM)
            DFB = (RCJ, KJK)
            XTG = (RFG, XSD)
            PKQ = (DBD, JXX)
            XPD = (GBL, VVR)
            FFC = (JLR, LBB)
            GMG = (KFH, PTP)
            QHJ = (SHM, FKM)
            MXD = (XGB, LTX)
            SGM = (FTS, GXX)
            LTX = (RMT, HGF)
            RNQ = (SPL, DKH)
            NHF = (VXC, HRS)
            KFR = (JXR, PFV)
            NRP = (XNN, MTB)
            VDR = (LDL, JCP)
            FSS = (KXJ, GKL)
            QVS = (MKN, BLH)
            SNN = (SBM, PNH)
            VXB = (LLG, BHP)
            BPK = (QVS, PPV)
            SMV = (TGJ, LPL)
            CXF = (XKV, PVR)
            PGC = (NBJ, KCK)
            VLG = (DDG, MBK)
            FKL = (HFC, XPP)
            KFC = (XNM, CNR)
            FCC = (BRK, GLT)
            HQC = (JKV, JHQ)
            QQV = (CPG, HQD)
            VVG = (XGJ, XGJ)
            SVK = (HVH, HNJ)
            XQT = (LQX, HNT)
            JDB = (PXM, THR)
            JHR = (XRF, DHN)
            HRS = (TFJ, PQH)
            SVQ = (XHX, QNG)
            KGN = (FCV, SBH)
            GJX = (MFG, LMS)
            NBJ = (GVB, RMS)
            VQJ = (JPB, PDF)
            TKX = (JMT, LGM)
            FLL = (GTX, HKP)
            MFT = (FKM, SHM)
            JCP = (SSP, VMD)
            LPC = (KCD, JDB)
            XHR = (XPP, HFC)
            FKM = (DLL, XBQ)
            MGF = (NFX, MQS)
            JXJ = (MMP, CBT)
            DFP = (TGM, RHV)
            SSM = (NTC, TMT)
            DLX = (PXF, SBG)
            NNB = (TNT, MMF)
            QNG = (RHP, QBT)
            FLR = (CJK, QRL)
            RCP = (KKG, NTD)
            GJV = (SLG, DGN)
            FDN = (TBR, QDT)
            JTG = (RFT, PRC)
            LGM = (DLX, FPT)
            QDT = (CXV, SGD)
            HKP = (BLB, PCL)
            GXB = (MNH, GXT)
            QDS = (LTF, FDX)
            LMJ = (GJV, HVV)
            TDQ = (PTP, KFH)
            SGD = (HTT, PNR)
            XPV = (BPM, CPH)
            PTP = (MKL, KSP)
            DKG = (DKK, KNQ)
            QCN = (XTD, GQM)
            LSV = (GDK, VVD)
            MLG = (RLS, RKG)
            GMX = (FKL, XHR)
            QSN = (PNH, SBM)
            CHQ = (KSB, CXF)
            RHN = (LTX, XGB)
            LVX = (JSF, MLT)
            XHM = (FLR, QVK)
            NCJ = (CPG, HQD)
            MQV = (HBD, LDK)
            VVD = (QBV, FKF)
            QCD = (GMG, TDQ)
            GTH = (DLJ, DQF)
            KFH = (KSP, MKL)
            NFC = (PKQ, MDK)
            NCD = (FNB, LLT)
            MKG = (HNS, DDK)
            BTV = (KBS, HKH)
            QGL = (QTF, CVS)
            DDB = (DVF, HXV)
            PXF = (CVH, LMJ)
            NFV = (XMN, BQR)
            DGN = (SNJ, PTL)
            HHL = (PHC, QQJ)
            XPR = (XGJ, LVQ)
            PCQ = (BRD, SFT)
            MMT = (RDS, SRS)
            DMR = (GJM, BQS)
            QPL = (XNQ, CMN)
            PDF = (JBH, CTX)
            CVR = (STX, MBQ)
            LPL = (SFL, PHQ)
            FXX = (PNX, PQL)
            KLS = (NHG, CHF)
            NHX = (TMN, JLJ)
            NND = (QHN, XBR)
            JTT = (QNG, XHX)
            LML = (FDX, LTF)
            PCS = (LKV, LXX)
            SRV = (KCD, JDB)
            NSK = (NBJ, KCK)
            MRF = (NKP, JKG)
            VPX = (NSN, SMB)
            GFT = (FPL, MSB)
            DFM = (XKQ, PPQ)
            PPQ = (PCS, BSH)
            JJC = (JHR, RQG)
            JBH = (BDV, QHL)
            KNQ = (KNP, MRL)
            TBS = (NTD, KKG)
            JXL = (RQG, JHR)
            GKL = (MJC, QBM)
            BQR = (LKQ, CJV)
            DGP = (VNT, JRB)
            DKR = (VBJ, GVF)
            JXN = (BXD, XLT)
            TVG = (HGR, GMX)
            XSD = (TLM, KPH)
            RMT = (GXB, DPT)
            FRL = (VVD, GDK)
            PNX = (NCD, KHN)
            VGK = (CNR, XNM)
            MQB = (KSF, LJS)
            KDC = (MSB, FPL)
            GCD = (CXS, PCC)
            XML = (VNT, JRB)
            TBL = (GCV, HDH)
            BRV = (DKK, KNQ)
            JKS = (XXB, ZZZ)
            TMC = (MDQ, QJF)
            TNT = (NQJ, SSM)
            FLF = (XSD, RFG)
            JRB = (FDF, HQC)
            PTL = (DBQ, MVG)
            BGQ = (KFR, HLB)
            XTD = (VRS, BRS)
            QXX = (SJD, CSH)
            FRX = (BQS, GJM)
            PQL = (NCD, KHN)
            LKV = (PNK, DKR)
            HQS = (VVG, XPR)
            VPQ = (NTL, VMQ)
            THR = (CBV, CVR)
            FST = (BQR, XMN)
            DMD = (CVS, QTF)
            HXV = (RCP, TBS)
            STM = (RFX, BFR)
            JTR = (JTL, JXN)
            FLG = (PFQ, CBJ)
            CKM = (KFB, HTP)
            NCT = (KJK, RCJ)
            DKH = (STB, TTB)
            QGA = (CGR, SGM)
            XNQ = (HJM, MLG)
            RGF = (LFN, FGC)
            QBV = (VPJ, RMN)
            QKJ = (KFC, VGK)
            CSB = (FDL, FDL)
            HFK = (DHS, SXC)
            SVP = (NPJ, BTL)
            GGR = (XTG, FLF)
            XKV = (NDH, JXJ)
            BMR = (FRX, DMR)
            VVQ = (BFN, TVK)
            HJM = (RKG, RLS)
            RLK = (PDF, JPB)
            DCX = (HNQ, BRZ)
            TBG = (RTL, SML)
            RMS = (FRL, LSV)
            FCV = (STM, HTC)
            SPR = (TBL, JGG)
            BHP = (LXK, KBL)
            NPJ = (XLV, PQR)
            NKF = (TBD, DKP)
            PDK = (JTK, JTK)
            PFQ = (RBX, JQL)
            LBB = (SVP, HTD)
            GTX = (BLB, PCL)
            SBH = (HTC, STM)
            BFR = (RJN, BTN)
            TCR = (TBL, JGG)
            MQS = (PXX, PLG)
            FRG = (XXB, XXB)
            STX = (XQL, HGM)
            HNJ = (HBN, SVN)
            SSL = (RDL, NST)
            HJS = (QCD, RLF)
            KBL = (SSL, MFR)
            CXS = (RHD, GQR)
            DFD = (NFP, FCC)
            VBJ = (HFK, GLB)
            DKP = (NNB, CQH)
            QSK = (TKX, CQT)
            BNV = (JTK, GTH)
            NBH = (LVX, GQB)
            QBM = (VNC, VPK)
            DBD = (HHR, RVQ)
            DFF = (BPM, CPH)
            LQX = (KJT, XXT)
            FGC = (QKQ, QSK)
            PXM = (CBV, CVR)
            JHQ = (CTB, DCX)
            NJN = (BGQ, FBP)
            TLL = (DDB, QRV)
            XSN = (THS, KJQ)
            KXJ = (QBM, MJC)
            GSC = (TQG, QXX)
            QJP = (PQL, PNX)
            PHH = (MXD, RHN)
            SSC = (NQX, FCH)
            XDF = (NLQ, VCR)
            BRD = (MQV, FRQ)
            AAA = (LRV, HJS)
            PTT = (FFC, HRC)
            VKF = (BFG, TGQ)
            HHA = (FLG, VHN)
            CNR = (TQX, VDV)
            TGM = (FLL, TNV)
            GLT = (DJB, QPL)
            MBK = (NBS, KTV)
            JPB = (JBH, CTX)
            RHP = (SQM, DJP)
            CHF = (CLQ, JGQ)
            FTL = (PRC, RFT)
            SKL = (XTD, GQM)
            RBN = (SPM, VLG)
            HGF = (GXB, DPT)
            BQS = (NSP, BTV)
            XHX = (RHP, QBT)
            CGR = (GXX, FTS)
            BLB = (QJP, FXX)
            CRT = (JLJ, TMN)
            PLG = (SLC, PDD)
            HRV = (FFC, HRC)
            MCM = (JXN, JTL)
            JGH = (MFG, LMS)
            NSP = (KBS, HKH)
            MFR = (NST, RDL)
            GMF = (TVK, BFN)
            SML = (LJL, XFP)
            MPL = (RDS, SRS)
            QHP = (SPM, VLG)
            FCH = (GDC, XRN)
            NRQ = (FTL, JTG)
            HMB = (CRC, FSS)
            KPH = (BNH, SLK)
            SRS = (MQB, GCN)
            RQM = (TGQ, BFG)
            FLN = (CSB, CSB)
            BSH = (LKV, LXX)
            CRC = (KXJ, GKL)
            CJV = (XDF, CHK)
            CGJ = (DGP, XML)
            SFL = (GJJ, HQS)
            PMR = (VPF, GKD)
            QTN = (MMT, MPL)
            KJQ = (RLK, VQJ)
            PBJ = (LFN, FGC)
            BTL = (PQR, XLV)
            SBX = (KFC, VGK)
            QQJ = (PDK, BNV)
            BRZ = (GNF, QQN)
            JTL = (XLT, BXD)
            SKG = (PRV, DBK)
            RQG = (DHN, XRF)
            DRC = (SFC, CHQ)
            TJQ = (NQX, FCH)
            PKS = (TMM, PMR)
            TRP = (GVD, FBL)
            LKQ = (XDF, CHK)
            RTL = (XFP, LJL)
            PPV = (MKN, BLH)
            MTB = (GCG, TMC)
            FPT = (PXF, SBG)
            LDK = (FDN, PFD)
            TND = (PPX, SVK)
            VPK = (NHF, JDX)
            HTP = (NKF, RPV)
            PMS = (SMB, NSN)
            GGG = (XKQ, PPQ)
            DLL = (KKN, QTN)
            JXX = (HHR, RVQ)
            KSF = (QPV, GDQ)
            XQD = (FGK, BNJ)
            RJF = (CSB, KTM)
            GCN = (KSF, LJS)
            JKP = (LML, QDS)
            XFP = (VPX, PMS)
            DPT = (GXT, MNH)
            DKK = (KNP, MRL)
            NKB = (DDK, HNS)
            CJK = (GSC, GBQ)
            NLM = (LDL, JCP)
            VMD = (DFD, XFM)
            NBX = (XJC, CGJ)
            PRS = (NLM, VDR)
            RMQ = (FRG, FRG)
            XNM = (TQX, VDV)
            LJC = (VDR, NLM)
            PHQ = (GJJ, HQS)
            KXN = (KHB, HHL)
            BXD = (CKM, XVL)
            KFB = (NKF, RPV)
            NFS = (PRL, RFL)
            RJN = (SFV, RRG)
            GQM = (VRS, BRS)
            KQH = (VBC, BSJ)
            DNR = (BMR, LJQ)
            HHR = (LPC, SRV)
            DLJ = (MHG, MHG)
            SMB = (RFJ, NJT)
            RHV = (FLL, TNV)
            NTD = (HBX, GVS)
            VNT = (FDF, HQC)
            PRC = (GMF, VVQ)
            FBP = (HLB, KFR)
            KTM = (FDL, LRZ)
            TQP = (QGL, DMD)
            SBM = (QPG, KXN)
            QVK = (CJK, QRL)
            XRF = (RGF, PBJ)
            NDH = (CBT, MMP)
            GVL = (RTL, SML)
            LRZ = (VHN, FLG)
            CQH = (MMF, TNT)
            SLK = (TJQ, SSC)
            QBT = (SQM, DJP)
            FNF = (LLG, BHP)
            STB = (KMS, NRP)
            HVV = (SLG, DGN)
            RFX = (BTN, RJN)
            MKL = (GVM, LVP)
            VDV = (BPK, CFB)
            CBT = (GJX, JGH)
            PPX = (HVH, HNJ)
            NQJ = (TMT, NTC)
            MJH = (MHP, XQT)
            LTA = (KLS, BFV)
            CSH = (BXQ, GHF)
            HTK = (XTG, FLF)
            VNC = (JDX, NHF)
            BFG = (MRC, NKK)
            KKG = (GVS, HBX)
            PNK = (GVF, VBJ)
            XXB = (LRV, HJS)
            DVF = (TBS, RCP)
            PNR = (NCJ, QQV)
            QKQ = (CQT, TKX)
            TTR = (SPL, DKH)
            BTN = (RRG, SFV)
            VLM = (PHH, SFH)
            TQG = (SJD, CSH)
            MHP = (LQX, HNT)
            NQX = (GDC, XRN)
            TQX = (BPK, CFB)
            TVB = (LHR, SMV)
            MBQ = (XQL, HGM)
            HXJ = (MQS, NFX)
            DHN = (PBJ, RGF)
            BFN = (MKG, NKB)
            PVR = (JXJ, NDH)
            QRL = (GBQ, GSC)
            GJM = (NSP, BTV)
            KQV = (NHH, CQK)
            KNP = (TKS, PKS)
            PDD = (FND, JKP)
            GXT = (VKF, RQM)
            XDC = (FTL, JTG)
            MMF = (SSM, NQJ)
            BFV = (CHF, NHG)
            DHS = (PHX, PHX)
            VXC = (PQH, TFJ)
            XLV = (TLL, RVS)
            PXX = (SLC, PDD)
            GVD = (DRC, DRC)
            THS = (VQJ, RLK)
            DBK = (PGC, NSK)
            KHB = (PHC, QQJ)
            TKS = (PMR, TMM)
            SVR = (NJN, TFP)
            HNQ = (QQN, GNF)
            MKN = (GGG, DFM)
            KRP = (NFM, NFC)
            SHM = (XBQ, DLL)
            NTL = (NCT, DFB)
            NRK = (QVK, FLR)
            HXL = (DSX, DSX)
            KSB = (XKV, PVR)
            NKK = (BKF, QMJ)
            MHG = (KLS, BFV)
            PNP = (XBR, QHN)
            CLQ = (TPX, NMV)
            RSG = (XDC, NRQ)
            BPM = (PCQ, QKX)
            PHC = (PDK, BNV)
            TNR = (KJV, QGC)
            XQL = (VPV, SVR)
            MHV = (QTS, RSG)
            XBR = (THC, JHT)
            BDX = (PHH, SFH)
            TBR = (CXV, SGD)
            RVQ = (SRV, LPC)
            NFM = (PKQ, MDK)
            FBL = (DRC, QSZ)
            TLM = (BNH, SLK)
            NHH = (SKS, GPP)
            FKF = (RMN, VPJ)
            XVL = (HTP, KFB)
            NHG = (CLQ, JGQ)
            FGK = (NBH, KBX)
            VMQ = (DFB, NCT)
            LXX = (DKR, PNK)
            SMJ = (QGB, NFS)
            LVP = (KDC, GFT)
            QGB = (PRL, RFL)
            BRX = (BNJ, FGK)
            BDV = (MRF, RXX)
            SJM = (BMR, LJQ)
            GBQ = (TQG, QXX)
            JXR = (XJT, CVX)
            TMN = (XSN, PQM)
            CQT = (LGM, JMT)
            CPG = (QHP, RBN)
            BKZ = (BFV, KLS)
            VVR = (HTK, GGR)
            XFM = (NFP, FCC)
            CVH = (GJV, HVV)
            VPS = (XQT, MHP)
            MDJ = (QHJ, MFT)
            SNF = (QTS, RSG)
            NKQ = (HMB, JVX)
            PRL = (FNF, VXB)
            CMN = (MLG, HJM)
            JQL = (NKQ, CNV)
            QSZ = (CHQ, SFC)
            HLR = (MHV, SNF)
            RDL = (TBG, GVL)
            GVM = (KDC, GFT)
            KTV = (CKK, VBK)
            SLC = (JKP, FND)
            QHL = (MRF, RXX)
            KKN = (MPL, MMT)
            TNV = (HKP, GTX)
            QMJ = (TVG, JCF)
            BSJ = (NFV, FST)
            BNJ = (NBH, KBX)
            XXQ = (GCD, CMQ)
            TGQ = (NKK, MRC)
            SVH = (NFC, NFM)
            LQB = (FRG, JKS)
            GNF = (HRV, PTT)
            PFV = (CVX, XJT)
            JTK = (DLJ, DLJ)
            KSP = (LVP, GVM)
            PFD = (TBR, QDT)
            MLT = (KSQ, KKJ)
            GVF = (HFK, GLB)
            JLS = (PPX, SVK)
            TBK = (CMQ, GCD)
            LJB = (QHJ, MFT)
            KSQ = (PST, TQP)
            SXC = (PHX, TRP)
            TTB = (NRP, KMS)
            VBX = (MHV, SNF)
            CXV = (PNR, HTT)
            XCT = (NFS, QGB)
            JBD = (KRP, SVH)
            JDX = (HRS, VXC)
            BNH = (TJQ, SSC)
            HQD = (RBN, QHP)
            RXX = (NKP, JKG)
            NTC = (NND, PNP)
            QTF = (TVQ, MHR)
            JND = (FCV, SBH)
            RDS = (MQB, GCN)
            FDL = (FLG, VHN)
            TGJ = (SFL, PHQ)
            KBX = (GQB, LVX)
            GQR = (TMD, KQH)
            LMS = (JBD, GSM)
            VPJ = (SMJ, XCT)
            LVQ = (HXL, VNK)
            GDK = (QBV, FKF)
            KCD = (PXM, THR)
            JGG = (GCV, HDH)
            NFP = (BRK, GLT)
            SFV = (TBK, XXQ)
            KJK = (QKN, XPD)
            CVX = (SBX, QKJ)
            GCG = (QJF, MDQ)
            FNB = (VFV, VPQ)
            CBV = (STX, MBQ)
            PNH = (QPG, KXN)
            HFC = (GSJ, RDT)
            GHF = (FLN, RJF)
            CKK = (BRV, DKG)
            LFN = (QKQ, QSK)
            MNH = (VKF, RQM)
            MMP = (GJX, JGH)
            HVB = (SSG, TNR)
            TMD = (BSJ, VBC)
            QTS = (NRQ, XDC)
            TVQ = (PTN, HVB)
            HTT = (QQV, NCJ)
            PQM = (KJQ, THS)
            XJT = (QKJ, SBX)
            PST = (QGL, DMD)
            XXT = (MBD, KQV)
            PCL = (QJP, FXX)
            MJC = (VPK, VNC)
            QPV = (JLS, TND)
            JLJ = (PQM, XSN)
            BKF = (TVG, JCF)
            SFT = (FRQ, MQV)
            XGJ = (HXL, HXL)
            JSF = (KKJ, KSQ)
            RMN = (XCT, SMJ)
            RPV = (TBD, DKP)
            XLT = (CKM, XVL)
            DJB = (CMN, XNQ)
            KCK = (GVB, RMS)
            LLT = (VFV, VPQ)
            CVS = (MHR, TVQ)
            NBS = (VBK, CKK)
            LJQ = (DMR, FRX)
            HKS = (TGM, RHV)
            HTC = (BFR, RFX)
            LDL = (VMD, SSP)
            QRV = (HXV, DVF)
            NST = (TBG, GVL)
            GQB = (JSF, MLT)
            KJT = (MBD, KQV)
            GSJ = (SVQ, JTT)
            FDX = (TVB, GXP)
            SSP = (XFM, DFD)
            FRQ = (LDK, HBD)
            JLR = (HTD, SVP)
            HNS = (SPR, TCR)
            MCZ = (SGM, CGR)
            GCV = (VPS, MJH)
            MDQ = (JJC, JXL)
            RKG = (NHX, CRT)
            TMT = (NND, PNP)
            SFC = (KSB, CXF)
            CQK = (SKS, GPP)
            FND = (LML, QDS)
            HVH = (HBN, SVN)
            CNV = (HMB, JVX)
            CMQ = (CXS, PCC)
            NLQ = (JTR, MCM)
            GDC = (RMQ, RMQ)
            PHX = (GVD, GVD)
            NKP = (HXJ, MGF)
            RRG = (XXQ, TBK)
            TFP = (BGQ, FBP)
            GXX = (LJB, MDJ)
        "#;
        assert_eq!(solve(input), 9064949303801);
    }
}