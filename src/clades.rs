use std::{
    boxed::Box,
    fmt::{self, Display},
};
use tabled::{
    object::{Columns, Rows},
    Disable, Modify, Panel, Table, Tabled, Width,
};

/// A telomeric repeat sequence, or sequences.
#[derive(Debug, Clone)]
pub struct Seq<'a>(pub Box<&'a [&'a str]>);

impl Seq<'_> {
    /// Get the sequence corresponding to an index.
    pub fn get(&self, index: usize) -> Option<&&str> {
        self.0.get(index)
    }
}

impl Display for Seq<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0.join(", ");
        write!(f, "{}", inner)
    }
}

/// All the relevant information about a
/// telomeric repeat sequence.
#[derive(Debug, Clone, Tabled)]
pub struct TelomereSeq<'a> {
    #[tabled(rename = "Clade")]
    /// The clade a telomeric repeat belongs to.
    pub clade: &'a str,
    #[tabled(rename = "Telomeric repeat units")]
    /// The actual telomeric repeat sequence(s).
    pub seq: Seq<'a>,
    /// How many different telomeric repeats counted
    /// for a clade.
    pub length: usize,
}

// automated input start

/// All the clades for which we have data.
pub static CLADES: &[&str] = &[
    "Accipitriformes",
    "Actiniaria",
    "Anura",
    "Apiales",
    "Aplousobranchia",
    "Asterales",
    "Buxales",
    "Caprimulgiformes",
    "Carangiformes",
    "Carcharhiniformes",
    "Cardiida",
    "Carnivora",
    "Caryophyllales",
    "Cheilostomatida",
    "Chiroptera",
    "Chlamydomonadales",
    "Coleoptera",
    "Crassiclitellata",
    "Cypriniformes",
    "Eucoccidiorida",
    "Fabales",
    "Fagales",
    "Forcipulatida",
    "Hemiptera",
    "Heteronemertea",
    "Hirudinida",
    "Hymenoptera",
    "Hypnales",
    "Labriformes",
    "Lamiales",
    "Lepidoptera",
    "Malpighiales",
    "Myrtales",
    "Odonata",
    "Orthoptera",
    "Pectinida",
    "Perciformes",
    "Phlebobranchia",
    "Phyllodocida",
    "Plecoptera",
    "Pleuronectiformes",
    "Poales",
    "Rodentia",
    "Rosales",
    "Salmoniformes",
    "Sapindales",
    "Solanales",
    "Symphypleona",
    "Syngnathiformes",
    "Trichoptera",
    "Trochida",
    "Venerida",
];

/// A function to get a telomeric repeat sequence
/// given a clade name.
pub fn return_telomere_sequence(clade: &str) -> TelomereSeq {
    let result = match clade {
        "Accipitriformes" => TelomereSeq {
            clade: "Accipitriformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Actiniaria" => TelomereSeq {
            clade: "Actiniaria",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Anura" => TelomereSeq {
            clade: "Anura",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Apiales" => TelomereSeq {
            clade: "Apiales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Aplousobranchia" => TelomereSeq {
            clade: "Aplousobranchia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Asterales" => TelomereSeq {
            clade: "Asterales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Buxales" => TelomereSeq {
            clade: "Buxales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Caprimulgiformes" => TelomereSeq {
            clade: "Caprimulgiformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Carangiformes" => TelomereSeq {
            clade: "Carangiformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Carcharhiniformes" => TelomereSeq {
            clade: "Carcharhiniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Cardiida" => TelomereSeq {
            clade: "Cardiida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Carnivora" => TelomereSeq {
            clade: "Carnivora",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Caryophyllales" => TelomereSeq {
            clade: "Caryophyllales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Cheilostomatida" => TelomereSeq {
            clade: "Cheilostomatida",
            seq: Seq(Box::new(&["AAACCCC"])),
            length: 1,
        },

        "Chiroptera" => TelomereSeq {
            clade: "Chiroptera",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Chlamydomonadales" => TelomereSeq {
            clade: "Chlamydomonadales",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Coleoptera" => TelomereSeq {
            clade: "Coleoptera",
            seq: Seq(Box::new(&["AACCT", "ACCTG", "AACAGACCCG", "AACCC"])),
            length: 4,
        },

        "Crassiclitellata" => TelomereSeq {
            clade: "Crassiclitellata",
            seq: Seq(Box::new(&["AAGGAC"])),
            length: 1,
        },

        "Cypriniformes" => TelomereSeq {
            clade: "Cypriniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Eucoccidiorida" => TelomereSeq {
            clade: "Eucoccidiorida",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Fabales" => TelomereSeq {
            clade: "Fabales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Fagales" => TelomereSeq {
            clade: "Fagales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Forcipulatida" => TelomereSeq {
            clade: "Forcipulatida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Hemiptera" => TelomereSeq {
            clade: "Hemiptera",
            seq: Seq(Box::new(&["AAACCACCCT", "AACCATCCCT"])),
            length: 2,
        },

        "Heteronemertea" => TelomereSeq {
            clade: "Heteronemertea",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Hirudinida" => TelomereSeq {
            clade: "Hirudinida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Hymenoptera" => TelomereSeq {
            clade: "Hymenoptera",
            seq: Seq(Box::new(&[
                "AAACCC",
                "AAAGAACCT",
                "AACCCAGACGC",
                "AACCCGAACCT",
                "AACCCTGACGC",
                "AAAATTGTCCGTCC",
                "AACCC",
                "AACCCCAACCT",
                "AAATGTGGAGG",
                "AACCCAGACCC",
                "ACCCAG",
                "AACCCAGACCT",
                "AACCCT",
                "ACGGCAGCG",
                "AACCT",
            ])),
            length: 15,
        },

        "Hypnales" => TelomereSeq {
            clade: "Hypnales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Labriformes" => TelomereSeq {
            clade: "Labriformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Lamiales" => TelomereSeq {
            clade: "Lamiales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Lepidoptera" => TelomereSeq {
            clade: "Lepidoptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Malpighiales" => TelomereSeq {
            clade: "Malpighiales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Myrtales" => TelomereSeq {
            clade: "Myrtales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Odonata" => TelomereSeq {
            clade: "Odonata",
            seq: Seq(Box::new(&["AACCC"])),
            length: 1,
        },

        "Orthoptera" => TelomereSeq {
            clade: "Orthoptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Pectinida" => TelomereSeq {
            clade: "Pectinida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Perciformes" => TelomereSeq {
            clade: "Perciformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Phlebobranchia" => TelomereSeq {
            clade: "Phlebobranchia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Phyllodocida" => TelomereSeq {
            clade: "Phyllodocida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Plecoptera" => TelomereSeq {
            clade: "Plecoptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Pleuronectiformes" => TelomereSeq {
            clade: "Pleuronectiformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Poales" => TelomereSeq {
            clade: "Poales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Rodentia" => TelomereSeq {
            clade: "Rodentia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Rosales" => TelomereSeq {
            clade: "Rosales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Salmoniformes" => TelomereSeq {
            clade: "Salmoniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Sapindales" => TelomereSeq {
            clade: "Sapindales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Solanales" => TelomereSeq {
            clade: "Solanales",
            seq: Seq(Box::new(&["AACCCTG"])),
            length: 1,
        },

        "Symphypleona" => TelomereSeq {
            clade: "Symphypleona",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Syngnathiformes" => TelomereSeq {
            clade: "Syngnathiformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Trichoptera" => TelomereSeq {
            clade: "Trichoptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Trochida" => TelomereSeq {
            clade: "Trochida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Venerida" => TelomereSeq {
            clade: "Venerida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        _ => panic!("{} is not yet accounted for in this pipeline.", clade),
    };
    result
}
// automated input end

/// Pretty print a table containing all the information about
/// telomeric repeats that we currently have.
pub fn print_table() {
    let mut clade_vec = Vec::new();

    for clade in CLADES {
        clade_vec.push(return_telomere_sequence(clade));
    }

    eprintln!(
        "{}",
        Table::new(&clade_vec)
            .with(
                Modify::new(Rows::new(1..clade_vec.len() - 1)).with(Width::wrap(30).keep_words())
            )
            .with(Disable::column(Columns::new(2..3)))
            .with(Panel::footer(
                "This table is created from a curated database of repeats. This database can be found in its raw form here: https://github.com/tolkit/telomeric-identifier/tree/main/clades/curated.csv"
            )).with(Width::wrap(60).keep_words())
    );
}
