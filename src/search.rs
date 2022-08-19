use crate::{utils, SubCommand};
use anyhow::{Context, Result};
use bio::io::fasta;
use std::fs::{create_dir_all, File};
use std::io::LineWriter;
use std::io::Write;
use std::str;

/// The entry point for `tidk search`.
pub fn search(matches: &clap::ArgMatches, sc: SubCommand) -> Result<()> {
    let input_fasta = matches
        .value_of("fasta")
        .context("Could not get the value of `fasta`.")?;
    let reader = fasta::Reader::from_file(input_fasta)?;

    let telomeric_repeat: String = matches
        .value_of_t("string")
        .context("Could not parse `string` as String.")?;
    let extension: String = matches
        .value_of_t("extension")
        .context("Could not parse `extension` as String.")?;

    eprintln!(
        "[+]\tSearching genome for telomeric repeat: {}",
        telomeric_repeat
    );

    let window_size: usize = matches
        .value_of_t("window")
        .context("Could not parse `window` as usize.")?;
    let outdir = matches
        .value_of("dir")
        .context("Could not get the value of `dir`.")?;
    let output = matches
        .value_of("output")
        .context("Could not get the value of `output`.")?;

    // create directory for output
    create_dir_all(format!("{}", outdir))?;

    // create file
    let file_name = format!(
        "{}/{}{}{}",
        outdir, output, "_telomeric_repeat_windows.", extension
    );
    let search_file = File::create(&file_name)?;
    let mut search_file = LineWriter::new(search_file);

    // add headers if extension/file type is a csv
    if extension == "tsv" {
        writeln!(
            search_file,
            "id\twindow\tforward_repeat_number\treverse_repeat_number\ttelomeric_repeat"
        )?;
    }

    // iterate over the fasta records
    for result in reader.records() {
        let record = result?;
        let id = record.id().to_owned();

        // fn window counter
        write_window_counts(
            record,
            &mut search_file,
            &telomeric_repeat,
            window_size,
            id.clone(),
            &extension,
        )?;

        eprintln!("[+]\tChromosome {} processed", id);
    }
    eprintln!("[+]\tFinished searching genome.");

    // optional log file
    sc.log(matches)?;

    Ok(())
}

/// Iterate over windows, counting occurrences of specified string
/// and write to file on the fly.
fn write_window_counts<T: std::io::Write>(
    sequence: bio::io::fasta::Record,
    file: &mut LineWriter<T>,
    telomeric_repeat: &str,
    window_size: usize,
    id: String,
    extension: &str,
) -> Result<()> {
    // get forward and reverse sequences, and length
    // to remove overlapping matches.
    let forward_telomeric_seq = telomeric_repeat;
    let reverse_telomeric_seq = utils::reverse_complement(forward_telomeric_seq);
    let telomeric_length = forward_telomeric_seq.len();

    // create the iterator in each loop iteration isnt costly is it?
    let windows = sequence.seq().chunks(window_size);
    // keep track of the window size
    let mut window_index = window_size;
    // iterate over windows
    for window in windows {
        // make window uppercase
        let windows_upper = str::from_utf8(window)?.to_uppercase();
        // for each window, find the motifs in this
        let forward_motif = utils::find_motifs(forward_telomeric_seq, &windows_upper);
        let reverse_motif = utils::find_motifs(&reverse_telomeric_seq, &windows_upper);

        // remove overlapping matches
        // not sure this is necessary, but thought it might be...
        let forward_motif_noverlap =
            utils::remove_overlapping_indexes(forward_motif, telomeric_length);
        let reverse_motif_noverlap =
            utils::remove_overlapping_indexes(reverse_motif, telomeric_length);

        // the number of matches for forward/reverse
        let forward_repeat_number = forward_motif_noverlap.len();
        let reverse_repeat_number = reverse_motif_noverlap.len();
        // write to file
        if extension == "tsv" {
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}",
                id,
                window_index,
                forward_repeat_number,
                reverse_repeat_number,
                forward_telomeric_seq
            )?;
        } else {
            // for bedgraph only four columns, and sum the forward & reverse for convenience
            writeln!(
                file,
                "{}\t{}\t{}\t{}",
                id,
                window_index - window_size,
                window_index,
                forward_repeat_number + reverse_repeat_number,
            )?;
        }
        // increment window
        window_index += window_size;
    }
    Ok(())
}
