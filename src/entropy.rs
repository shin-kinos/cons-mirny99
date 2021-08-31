
use std::collections::HashMap;
use std::f64;
use std::cmp::min;
use crate::weighting::SYMBOL;

	/*
	 * Define a pseudo symbol list to classify AA by 7 stereochemical properties.
	 * This variable is used only in this scope.
	*/
	static mut SYMBOL_07 : Vec<char> = Vec::new(); /* vec![ 'A', 'B', 'C', 'D', 'E', 'F', 'G' ]; */

pub fn shannon_entropy( site_list : &Vec<String>, weight_list : &Vec<f64>, gap_pen_list : &Vec<f64>, arg_s : &String ) -> Vec<f64>
{
	unsafe {
		SYMBOL_07 = "ABCDEFG".chars().collect();
	}

	let num_site : usize = ( *site_list ).len();

	let mut shan_ent_list : Vec<f64> = vec![ 0.0; num_site ];

	for i in 0 .. num_site {
		if ( *arg_s ) == "21" {
			let shan_ent : f64 = cal_shan_ent( &( *site_list )[ i ], weight_list, ( *gap_pen_list )[ i ] );
			shan_ent_list[ i ] += shan_ent;
		} else if ( *arg_s ) == "7" || ( *arg_s ) == "07" {
			let shan_ent : f64 = cal_shan_ent_07( &( *site_list )[ i ], weight_list, ( *gap_pen_list )[ i ] );
			shan_ent_list[ i ] += shan_ent;
		}
	}

	shan_ent_list
}

///////////////////////////////////////////////////////////////////////////////////////////
// SHANNON'S ENTROPY WITH 21 AMINO ACID TYPES
///////////////////////////////////////////////////////////////////////////////////////////

fn cal_shan_ent( site_arg : &String, weight_list : &Vec<f64>, gap_penalty : f64 ) -> f64
{
	let site : Vec<char> = ( *site_arg ).chars().collect();
	//println!( "site : {:?}", site );

	/* Calculate frequency in a site taking account of sequence weighting. */
	let weighted_freq : HashMap<char, f64> = weighted_freq_count( &site, weight_list );

	//let n : usize = site.len();

	let mut shan_ent : f64 = 0.0;

	unsafe {
		for aa in SYMBOL.iter() {
			shan_ent += weighted_freq[ aa ] * ( weighted_freq[ aa ] ).log2();
		}
	}

	shan_ent = -1.0 * shan_ent;

	/*
	 * Normalize the Shannon's entropy score.
	 * Conserved (low entropy) sites score 1 and diverse (high entropy) sites score 0.
	*/
	let denom : f64 = ( min( weighted_freq.len(), site.len() ) as f64 ).log2();
	let mut shan_ent_norm : f64 = 1.0 - ( shan_ent / denom );

	/* Add a gap penalty. */
	shan_ent_norm = shan_ent_norm * gap_penalty;

	//println!( "\nShannon entropy : {}", shan_ent_norm );

	shan_ent_norm
}

fn weighted_freq_count( site : &Vec<char>, weight_list : &Vec<f64> ) -> HashMap<char, f64>
{
	let len_site : usize = ( *site ).len();

	/* Define the pseudocount (10e-7). */
	let pseudo_count : f64 = 0.0000001;

	/* Define a hashmap to count AA frequency in a site. */
	let mut freq : HashMap<char, f64> = HashMap::new();
	unsafe {
		for aa in SYMBOL.iter() { freq.insert( *aa, pseudo_count ); }
	}
	//println!( "{:?}", freq );

	/*
	 * Count a frequency of each AA in a site taking accont of sequence weighting.
	 * Add a weighting score instead of simple increment (+1.0).
	 * aa               = One letter of AA in a site.
	 * add              = Weighting score add instead of 1.0.
	 * weight_list[ i ] = Weighting score of i th sequence.
	 * freq             = AA - weighted frequency hashmap.
	*/
	for i in 0 .. len_site {
		let aa  : char = ( *site )[ i ];
		let add : f64  = freq[ &aa ] + ( *weight_list )[ i ];
		freq.insert( aa, add );
	}
	//println!( "{:?}", freq );

	freq
}

///////////////////////////////////////////////////////////////////////////////////////////
// SHANNON'S ENTROPY WITH 7 AMINO ACID TYPES
///////////////////////////////////////////////////////////////////////////////////////////

fn cal_shan_ent_07( site_arg : &String, weight_list : &Vec<f64>, gap_penalty : f64 ) -> f64
{
	let mut site : Vec<char> = ( *site_arg ).chars().collect();
	//println!( "site : {:?}", site );

	/* 
	 * Convert amino acids in a site into 7 types of stereochemical property.
	 * A = Aliphatic.
	 * B = Aromatic.
	 * C = Polar.
	 * D = Positive.
	 * E = Negative.
	 * F = Special conformations.
	 * G = Gap.
	*/
	for i in 0 .. site.len() {
		match ( *site )[ i ] {
			'A' | 'V' | 'L' | 'I' | 'M' | 'C' => ( *site )[ i ] = 'A',
			'F' | 'W' | 'Y' | 'H'             => ( *site )[ i ] = 'B',
			'S' | 'T' | 'N' | 'Q'             => ( *site )[ i ] = 'C',
			'K' | 'R'                         => ( *site )[ i ] = 'D',
			'D' | 'E'                         => ( *site )[ i ] = 'E',
			'G' | 'P'                         => ( *site )[ i ] = 'F',
			_                                 => ( *site )[ i ] = 'G',
		}
	}
	site.shrink_to_fit();

	/* Calculate frequency in a site taking account of sequence weighting. */
	let weighted_freq : HashMap<char, f64> = weighted_freq_count_07( &site, weight_list );

	//let n : usize = site.len();

	let mut shan_ent : f64 = 0.0;

	unsafe {
		for aa in SYMBOL_07.iter() {
			shan_ent += weighted_freq[ aa ] * ( weighted_freq[ aa ] ).log2();
		}
	}

	shan_ent = -1.0 * shan_ent;

	/*
	 * Normalize the Shannon's entropy score.
	 * Conserved (low entropy) sites score 1 and diverse (high entropy) sites score 0.
	*/
	let denom : f64 = ( min( weighted_freq.len(), site.len() ) as f64 ).log2();
	let mut shan_ent_norm : f64 = 1.0 - ( shan_ent / denom );

	/* Add a gap penalty. */
	shan_ent_norm = shan_ent_norm * gap_penalty;

	//println!( "\nShannon entropy : {}", shan_ent_norm );

	shan_ent_norm
}

fn weighted_freq_count_07( site : &Vec<char>, weight_list : &Vec<f64> ) -> HashMap<char, f64>
{
	let len_site : usize = ( *site ).len();

	/* Define the pseudocount (10e-7). */
	let pseudo_count : f64 = 0.0000001;

	/* Define a hashmap to count AA frequency in a site. */
	let mut freq : HashMap<char, f64> = HashMap::new();

	unsafe {
		for aa in SYMBOL_07.iter() { freq.insert( *aa, pseudo_count ); }
	}
	//println!( "{:?}", freq );

	/*
	 * Count a frequency of each AA in a site taking accont of sequence weighting.
	 * Add a weighting score instead of simple increment (+1.0).
	 * aa               = One letter of AA in a site.
	 * add              = Weighting score add instead of 1.0.
	 * weight_list[ i ] = Weighting score of i th sequence.
	 * freq             = AA - weighted frequency hashmap.
	*/
	for i in 0 .. len_site {
		let aa  : char = ( *site )[ i ];
		let add : f64  = freq[ &aa ] + ( *weight_list )[ i ];
		freq.insert( aa, add );
	}
	//println!( "{:?}", freq );

	freq
}
