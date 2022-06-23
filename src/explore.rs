use crate::{utils, SubCommand};
use anyhow::{Context, Result};
use bio::io::fasta;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::str;
use std::sync::mpsc::channel;

// function called from main.rs
// TODO: sort out output file formats.

/// The function called from `tidk explore`. It takes the [`clap::Argmatches`]
/// from the user and also a [`SubCommand`].
pub fn explore(matches: &clap::ArgMatches, sc: SubCommand) -> Result<()> {
    // parse arguments from main
    let input_fasta = matches
        .value_of("fasta")
        .context("Could not get the value of `fasta`.")?;
    let length = matches
        .value_of_t("length")
        .context("Could not parse `length` as usize.")?;

    // if length is not set, these are the lengths (and length itself is set to zero)
    let minimum = matches
        .value_of_t("minimum")
        .context("Could not parse `minimum` as usize")?;
    let maximum: usize = matches
        .value_of_t("maximum")
        .context("Could not parse `maximum` as usize")?;

    let threshold = matches
        .value_of_t("threshold")
        .context("Could not parse `threshold` as i32")?;

    let outdir = matches
        .value_of("dir")
        .context("Could not get the value of `dir`.")?;
    let output = matches
        .value_of("output")
        .context("Could not get the value of `output`.")?;
    let extension: String = matches
        .value_of_t("extension")
        .context("Could not parse `extension` as String")?;

    let dist_from_chromosome_end = matches
        .value_of_t("distance")
        .context("Could not parse `distance` as usize")?;

    let verbose = matches.is_present("verbose");

    // create directory for output
    create_dir_all(format!("{}", outdir))?;

    // create file
    let file_name = format!(
        "{}/{}{}{}",
        outdir, output, "_telomeric_locations.", extension
    );
    let explore_file = File::create(&file_name)?;
    let mut explore_file = LineWriter::new(explore_file);

    let putative_telomeric_file = format!("{}/{}{}", outdir, output, ".txt");
    let putative_telomeric_file_txt = File::create(&putative_telomeric_file)?;
    let mut putative_telomeric_file_txt = LineWriter::new(putative_telomeric_file_txt);

    // add header to txt file
    writeln!(
        putative_telomeric_file_txt,
        "telomeric_repeat\treverse_complement\tfrequency"
    )?;

    // to report the telomeres...
    let mut output_vec: Vec<FormatTelomericRepeat> = Vec::new();
    let mut output_vec_bed: Vec<TsvTelomericRepeat> = Vec::new();
    // i.e. if you chose a length, as opposed to a minmum/maximum
    if length > 0 {
        println!(
            "[+]\tExploring genome for potential telomeric repeats of length: {}",
            length
        );
        let reader = fasta::Reader::from_file(input_fasta)?;

        // try parallelising
        let (sender, receiver) = channel();

        reader
            .records()
            .par_bridge()
            .for_each_with(sender, |s, record| {
                let record = record.expect("[-]\tError during fasta record parsing.");
                let id = record.id().to_owned();
                let seq_len = record.seq().len();

                let indexes = chunk_fasta(record, length);
                let adjacents = calculate_indexes(indexes);
                let formatted =
                    generate_explore_data(adjacents, id.clone(), length, verbose).unwrap_or(vec![]);

                s.send(
                    merge_rotated_repeats(
                        formatted,
                        length,
                        &id,
                        threshold,
                        seq_len,
                        dist_from_chromosome_end,
                        verbose,
                    )
                    .unwrap_or(Output {
                        telomeric_repeats: vec![],
                        bed_file: vec![],
                    }),
                )
                .expect("[-]\tDid not send.");
            });

        // this bit is a little chaotic
        // collect output into a vector
        let output: Vec<Output> = receiver.iter().collect();

        // so it can be cloned here
        // to extract repeats
        let mut telomeric_repeats = output
            .clone()
            .iter()
            .map(|a| a.telomeric_repeats.clone())
            .flatten()
            .collect();
        // and appended to the output vec
        output_vec.append(&mut telomeric_repeats);

        // and also cloned here
        let mut bed_file: Vec<TsvTelomericRepeat> = output
            .clone()
            .iter()
            .map(|a| a.bed_file.clone())
            .flatten()
            .collect();
        // to get a bed file of all potential repeat locations.
        output_vec_bed.append(&mut bed_file);
    } else {
        // if a range was chosen.
        println!(
            "[+]\tExploring genome for potential telomeric repeats between lengths {} and {}.",
            minimum, maximum
        );
        for length in minimum..maximum + 1 {
            println!("[+]\t\tFinding telomeric repeat length: {}", length);

            // have to call reader in the loop, as otherwise `reader` doesn't live long enough.
            // I expect it's not an expensive call anyway.
            let reader = fasta::Reader::from_file(input_fasta)?;

            // try parallelising
            let (sender, receiver) = channel();
            reader
                .records()
                .par_bridge()
                .for_each_with(sender, |s, record| {
                    let record = record.expect("[-]\tError during fasta record parsing.");
                    let id = record.id().to_owned();
                    let seq_len = record.seq().len();

                    let indexes = chunk_fasta(record, length);
                    let adjacents = calculate_indexes(indexes);
                    let formatted = generate_explore_data(adjacents, id.clone(), length, verbose)
                        .unwrap_or(vec![]);

                    s.send(
                        merge_rotated_repeats(
                            formatted,
                            length,
                            &id,
                            threshold,
                            seq_len,
                            dist_from_chromosome_end,
                            verbose,
                        )
                        .unwrap_or(Output {
                            telomeric_repeats: vec![],
                            bed_file: vec![],
                        }),
                    )
                    .expect("Did not send!");
                });
            // this bit is a little chaotic
            // collect output into a vector
            let output: Vec<Output> = receiver.iter().collect();

            // so it can be cloned here
            // to extract repeats
            let mut telomeric_repeats = output
                .clone()
                .iter()
                .map(|a| a.telomeric_repeats.clone())
                .flatten()
                .collect();
            // and appended to the output vec
            output_vec.append(&mut telomeric_repeats);

            // and also cloned here
            let mut bed_file: Vec<TsvTelomericRepeat> = output
                .clone()
                .iter()
                .map(|a| a.bed_file.clone())
                .flatten()
                .collect();
            // to get a bed file of all potential repeat locations.
            output_vec_bed.append(&mut bed_file);
        }
    }
    println!("[+]\tFinished searching genome");
    println!("[+]\tGenerating output");
    // print likely telomeric repeat
    // costly calculation if threshold is too low.
    get_telomeric_repeat_estimates(&mut output_vec, &mut putative_telomeric_file_txt);
    // write the bed file
    writeln!(
        explore_file,
        "id\tstart_pos\tend_pos\trepeat_number\trepeat_sequence\tsequence_length"
    )?;
    for line in output_vec_bed {
        writeln!(
            explore_file,
            "{}\t{}\t{}\t{}\t{}\t{}",
            line.id, line.start, line.end, line.count, line.sequence, line.seq_len
        )?;
    }

    // optional log file
    sc.log(matches)?;

    Ok(())
}

/// A chunked fasta segment with a position and a sequence.
/// We split the fasta into chunks of size k, where k is the
/// potential telomeric repeat length. Consecutive iterations
/// of these chunks are compared for equality.
pub struct ChunkedFasta {
    /// Position of the sequence in the
    /// fasta file.
    pub position: usize,
    /// The sequence itself.
    pub sequence: String,
}

/// Chunk a fasta into a [`Vec<ChunkedFasta`], i.e. split a fasta into chunks
/// and compare adjacent chunks for equality. Store the positions and sequences
/// if they are equivalent.
fn chunk_fasta(sequence: bio::io::fasta::Record, chunk_length: usize) -> Vec<ChunkedFasta> {
    let chunks = sequence.seq().chunks(chunk_length);
    let chunks_plus_one = sequence.seq()[chunk_length..sequence.seq().len()].chunks(chunk_length);

    // store the index positions and adjacent equivalent sequences.
    let mut indexes = Vec::new();
    // need this otherwise we lose the position in the sequence.
    // need to check this is actually correct.
    let mut pos = 0;

    // this is the heavy lifting.
    for (_i, (a, b)) in chunks.zip(chunks_plus_one).enumerate() {
        // if chunk contains N, skip.
        if a.contains(&78) || b.contains(&78) {
            pos += chunk_length;
            continue;
        } else if a == b {
            indexes.push(ChunkedFasta {
                position: pos,
                sequence: str::from_utf8(a).unwrap().to_uppercase(),
            });
        }
        pos += chunk_length;
    }
    indexes
}

// takes the positions from chunk_fasta
// take the current iteration position away from next iteration position

/// Hold information about the repeat runs.
pub struct RepeatRuns {
    /// The position of the repeat.
    pub position: usize,
    /// position[i + 1] - position[i].
    pub subtracted_position: usize,
    /// The sequence.
    pub sequence: String,
}

/// Iterate over our [`Vec<ChunkedFasta`] on the `i`th and `i` + 1th
/// iterations. Subtract the latter position from the former position.
fn calculate_indexes(indexes: Vec<ChunkedFasta>) -> Vec<RepeatRuns> {
    let mut adjacent_indexes = Vec::new();
    for i in 1..indexes.len() {
        if i + 1 < indexes.len() {
            adjacent_indexes.push(RepeatRuns {
                position: indexes[i].position,
                subtracted_position: indexes[i + 1].position - indexes[i].position,
                sequence: indexes[i].sequence.clone(),
            })
        }
    }
    adjacent_indexes
}

/// Holds information about putative telomeric
/// repeats and where they are in the genome.
#[derive(Debug, Clone)]
pub struct TelomericRepeatExplore {
    /// Start of the telomeric repeat chunk.
    pub start: usize,
    /// End of the telomeric repeat chunk.
    pub end: usize,
    /// How many times the telomeric repeat
    /// unit appeared in a given region.
    pub count: i32,
    /// The sequence of the telomeric repeat.
    pub sequence: String,
    /// The length of the telomeric repeat.
    pub sequence_len: usize,
}

// inital data generation, using the indexes above
// if there are a run of indexes == chunk_size,
// these are the repeats we are looking for.

/// This function generates the inital telomeric repeat
/// data. If there are runs of indexes which are equal to
/// the chunk size, then these are the repeats we are looking
/// for.
fn generate_explore_data(
    adjacent_indexes: Vec<RepeatRuns>,
    id: String,
    chunk_length: usize,
    verbose: bool,
) -> Option<Vec<TelomericRepeatExplore>> {
    if adjacent_indexes.is_empty() {
        // verbosity flag
        if verbose {
            eprintln!(
                "[-]\t\tChromosome {}: No consecutive repeats of length {} were identified.",
                id, chunk_length
            );
        }
        return None;
    }
    // collect all of this vector into a nice format to print for the moment.
    // iteration
    let mut it = 0;
    let mut count = 0;
    // threshold could become an input parameter
    // this is the first level of filtering which is useful
    // as most matches only occur twice (once repeated)
    // this local threshold parameter is quite interesting, too low and you get too much
    // output, but too high and you miss information.
    let local_threshold = 0;
    let mut start_pos = 0;
    let mut start;
    let mut end;
    let mut potential_telomeric_repeats = Vec::new();

    loop {
        start = adjacent_indexes[start_pos].position;
        if it == adjacent_indexes.len() - 1 {
            break;
        }
        if adjacent_indexes[it].subtracted_position == chunk_length {
            count += 1;
            it += 1;
        } else {
            end = adjacent_indexes[it].position;
            if count > local_threshold {
                potential_telomeric_repeats.push(TelomericRepeatExplore {
                    start: start,
                    end: end,
                    count: count,
                    sequence: adjacent_indexes[it].sequence.clone(),
                    sequence_len: chunk_length,
                });
            }

            start_pos = it;
            count = 1;
            it += 1;
        }
    }
    Some(potential_telomeric_repeats)
}

// basically pretty prints generate_explore_data
// but crucially aggregates runs of records which are
// string rotations of one another, yielding better summaries.

/// A formatted telomeric repeat.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormatTelomericRepeat {
    /// The sequence.
    sequence: String,
    /// How many telomeric repeats there were in
    /// a consecutive run.
    count: i32,
    /// Length of the sequence.
    sequence_len: usize,
}

/// Hold TSV data.
#[derive(Debug, Clone)]
pub struct TsvTelomericRepeat {
    /// The fasta record ID.
    id: String,
    /// Start of the telomeric repeat interval.
    start: usize,
    /// End of the telomeric repeat interval.
    end: usize,
    /// How many telomeric repeats in the interval.
    count: i32,
    /// The sequence.
    sequence: String,
    /// The length of the sequence.
    seq_len: usize,
}

/// The struct sent through the parallel iterators.
#[derive(Debug, Clone)]
pub struct Output {
    /// Vector of formatted telomeric repeats.
    telomeric_repeats: Vec<FormatTelomericRepeat>,
    /// Holds the data which will be printed as a TSV/bedgraph.
    bed_file: Vec<TsvTelomericRepeat>,
}

/// A function to merge telomeric repeats which are different
/// in sequence, but equivalent once rotated or reverse complement
/// and rotated, i.e. merge canonical telomeric repeats.
fn merge_rotated_repeats<'a>(
    data: Vec<TelomericRepeatExplore>,
    chunk_length: usize,
    id: &'a str,
    threshold: i32,
    seq_len: usize,
    dist_from_chromosome_end: usize,
    verbose: bool,
) -> Option<Output> {
    let mut output_vec: Vec<FormatTelomericRepeat> = Vec::new();
    let mut output_vec_tsv: Vec<TsvTelomericRepeat> = Vec::new();

    // check for absence of data in the vector
    if data.is_empty() {
        if verbose {
            eprintln!(
                "[-]\t\tChromosome {}: No consecutive repeats of length {} were identified.",
                id.clone(),
                chunk_length
            );
        }
        return None;
    }

    // keep track of iterations
    let mut it = 0;
    // initiate count as the count of the first element
    let mut count = data[0].count;
    // increment this only when no more string rotations are found
    let mut start_index = 0;
    // start and end chromosome positions of aggregated repeats.
    let mut start;
    let mut end;

    loop {
        // the starting value for the first result
        start = data[start_index].start;
        end = data[it].end;
        // explicit break in the loop
        if it == data.len() - 1 {
            // if all telomere repeat to the end, this is not printed.
            if count > threshold {
                if start < dist_from_chromosome_end || end > seq_len - dist_from_chromosome_end {
                    // to output later
                    output_vec_tsv.push(TsvTelomericRepeat {
                        id: id.to_owned(),
                        start,
                        end,
                        count,
                        sequence: data[it].sequence.clone(),
                        seq_len: chunk_length,
                    });
                    // and collect for guessing telomeric repeat
                    output_vec.push(FormatTelomericRepeat {
                        sequence: data[it].sequence.clone(),
                        count: count,
                        sequence_len: chunk_length,
                    });
                }
            }
            break Some(Output {
                telomeric_repeats: output_vec,
                bed_file: output_vec_tsv,
            });
        }
        // if consecutive sequences are rotations
        if utils::string_rotation(&data[it].sequence, &data[it + 1].sequence) {
            // increment count by number of counts in next iteration
            count += data[it + 1].count;
            it += 1;
        } else {
            end = data[it].end;
            if count > threshold {
                if start < dist_from_chromosome_end || end > seq_len - dist_from_chromosome_end {
                    // to output later
                    output_vec_tsv.push(TsvTelomericRepeat {
                        id: id.to_owned(),
                        start,
                        end,
                        count,
                        sequence: data[it].sequence.clone(),
                        seq_len: chunk_length,
                    });
                    // and collect for guessing telomeric repeat
                    output_vec.push(FormatTelomericRepeat {
                        sequence: data[it].sequence.clone(),
                        count,
                        sequence_len: chunk_length,
                    });
                }
            }
            it += 1;
            start_index = it;
            count = data[it].count;
        }
    }
}

/// Takes the final aggregation of potential telomeric repeats across
/// chromosomes and also potentially across different lengths and tries
/// to find the most likely telomeric repeat. See [`utils::format_telomeric_repeat()`]
/// for the explanation of the formatting.
fn get_telomeric_repeat_estimates<T: std::io::Write>(
    telomeric_repeats: &mut Vec<FormatTelomericRepeat>,
    putative_telomeric_file: &mut LineWriter<T>,
) {
    if telomeric_repeats.is_empty() {
        eprintln!("[-]\tNo potential telomeric repeats found.");
        return;
    }

    // we need to compare all elements against all others
    let mut map: HashMap<String, i32> = HashMap::new();
    // so we don't compare the same thing twice.
    let mut tracker: Vec<usize> = Vec::new();
    // create all combinations of indices
    let it = (0..telomeric_repeats.len()).combinations(2);

    // iterate over combinations
    for comb in it {
        // if the combination is a string rotation (or its reverse complement)
        // then combine
        if utils::string_rotation(
            &telomeric_repeats[comb[0]].sequence,
            &telomeric_repeats[comb[1]].sequence,
        ) || utils::string_rotation(
            &utils::reverse_complement(&telomeric_repeats[comb[0]].sequence),
            &telomeric_repeats[comb[1]].sequence,
        ) || utils::string_rotation(
            &utils::reverse_complement(&telomeric_repeats[comb[1]].sequence),
            &telomeric_repeats[comb[0]].sequence,
        ) {
            // if comb[0] || comb[1] not in tracker...
            // as we already added the contents of the tracked telomeric repeats
            // we do not want to count them again.
            if !tracker.contains(&comb[0]) && !tracker.contains(&comb[1]) {
                let count = map
                    // relies on the telomeric repeat string resolving to a 'canonical'
                    // or unique form of the string, see utils::lms()
                    .entry(utils::lms(
                        &telomeric_repeats[comb[0]].sequence,
                        &telomeric_repeats[comb[1]].sequence,
                    ))
                    .or_insert(telomeric_repeats[comb[0]].count + telomeric_repeats[comb[1]].count);
                *count += telomeric_repeats[comb[0]].count + telomeric_repeats[comb[1]].count;

                tracker.push(comb[0]);
                tracker.push(comb[1]);
            }
        }
    }

    let mut count_vec: Vec<_> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut it = 0;
    for (seq, count) in count_vec {
        if it == 0 {
            println!(
                "[+]\tThe likely telomeric repeat is: {}, found {} times.",
                seq, count
            );
        }
        writeln!(
            putative_telomeric_file,
            "{}\t{}\t{}",
            seq,
            utils::reverse_complement(seq),
            count
        )
        .unwrap_or_else(|_| println!("[-]\tError in writing to file."));
        it += 1;
    }
}
