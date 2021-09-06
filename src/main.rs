
use std::time::Instant;
use colored::*;

mod options;
mod fasta;
mod weighting;
mod gap;
mod result;
mod entropy;
mod error;

fn main()
{
	/* Elapsed time : Start */
	let time = Instant::now();

	println!( "\nCalculate sequence weighting.\n" );

	/* Set options. */
	let opts = options::Options::new();
	opts.show_parameter();

	/* Read an input file and get FASTA information. */
	let data = fasta::Fasta::new( &( opts.input ) );
	data.check_fasta_info();

	/*
	println!( "Inputfile content :\n" );
	for i in 0 .. ( data.sequence ).len() {
		println!( "Title    {} : {}", i + 1, ( data.title )[ i ] );
		println!( "Sequence {} : {}", i + 1, ( data.sequence )[ i ] );
	}
	*/

	/* Get site information to calculate Henikoff weighting factor. */
	let site_list : Vec<String> = data.get_site_list();

	/*
	println!( "\nSite content :\n" );
	for i in 0 .. site_list.len() {
		println!( "Site {} : {}", i + 1, site_list[ i ] );
	}
	*/

	//println!( "fn main(), &seq_list  : {:p}", &( data.sequence ) );
	//println!( "fn main(), &site_list : {:p}", &site_list );

	let weight_list : Vec<f64> = weighting::seq_weight( &( data.sequence ), &site_list, &( opts.method ), &( opts.tolerate ) );

	/*
	println!( "\nSequence weighting :\n" );
	for i in 0 .. weight_list.len() {
		println!( "Weight of Sequence {} : {}", i + 1, weight_list[ i ] );
		sum_weight += weight_list[ i ];
	}
	*/

	/* Calculate gap penalties taking acconut of sequence weighting. */
	let gap_pen_list : Vec<f64> = gap::weight_gap_penalty( &site_list, &weight_list );

	/*
	for i in 0 .. gap_pen_list.len() {
		println!( "[ Gap penalty ] Site {} : {:.4}", i + 1, gap_pen_list[ i ] );
	}
	*/

	let shan_ent_list : Vec<f64> = entropy::shannon_entropy( &site_list, &weight_list, &gap_pen_list, &( opts.stereo ) );

	/*
	for i in 0 .. shan_ent_list.len() {
	println!( "[ Shannon's entropy ] Site {} : {:.3}", i + 1, shan_ent_list[ i ] );
	}
	*/

	/* Show result. */
	result::show_result( &site_list, &shan_ent_list, &( opts.colorize ) );

	/* Save result. */
	result::save_result( &site_list, &shan_ent_list, &( opts.output ) );

	println!( "{}", "\nProgram completed !!!\n".green() );

	/* Elapsed time : End */
	println!( "Total elapsed time (sec) : {}.{:.3}", time.elapsed().as_secs(), time.elapsed().subsec_nanos() / 1_000_000 );
}
