# cons-re 
A Rust program that calculates conservation score a site in Multiple Sequence Alignment (MSA) by Relative Entropy. 

## Description 
* This program calculates conservation score in each site of MSA. 
* The scoring measure is Relative Entropy (RE) applied by Wang and Samudrala [1]. 
* It takes account of background distribution, sequence weighting and gap penalty. 

## Dependencies 

* `colored` ( https://github.com/mackwic/colored ) 

``` 
[dependencies]
colored = "2.0"
``` 

## Installation 

You can compile this program by using `Cargo`. 📦🦀

[e. g.] 

``` 
% cd cons-re-main
% cargo build --release
``` 
Then the object file will be generated in `./target/release` directory. 

## Scoring method 

### Conservation score 
The conservation score is calculated by using RE as follows: 

(image) 

where ***p*** is the amino acid probability of site ***C*** and ***q*** is a previously given back ground distribution. ⚠️NOTE that gaps are ignored when it constructs a site distribution. 

### Sequence weighting 
This program supports two types of sequence weighting: 

1. The Position-Based method by Henikoff and Henikoff [2].
2. The Distance-Based method by Vingron and Argos [3]. 

### Gap penalty 
The gap penalties are given as follows:

(image) 

where ***L*** is the length of a site (or number of the sequences in MSA). The gap penalty of site ***i*** is given by calculating sum of weighting factors assigned to the gaps in sequence ***j***. 

### Back ground distribution 
RE requires a background distribution. In this program, there are three types of background distributions you can choose: 

1. BLOSUM62 
2. AA composition of Swiss-Prot (from UniProt Knowledge Base) 
3. Non-biassed distribution (equal rate at 0.05)  


Gaps are ignored as well as site distributions.  

## Input file format
Aligned Multi-FASTA format of amino acid sequences. ⚠️ NOTE that nucleotide sequences are not supported.

See some example input files in `demo` directory.

## Usage
Major arguments:

`-i` : Input filename in aligned Multi-FASTA format, REQUIRED.

`-o` : Output filename, REQUIRED.

`-w` : Method of sequence weighting ( "hen" or "va", default "hen" ).

`-b` : Background distribution (default "blosum62").

[e. g.]

```
% ./cons-re -i input.fasta -o output.txt -v va -c yes -t no
```
Type `-h` to see other available options.

##Output
Number`\t`Conservation score`\t`Composition of the site

[e.g.]

(image)

## References 
1. Wang, Kai, and Ram Samudrala. "Incorporating background frequency improves entropy-based residue conservation measures." BMC bioinformatics 7.1 (2006): 1-8.
2. Henikoff, Steven, and Jorja G. Henikoff. "Position-based sequence weights." Journal of molecular biology 243.4 (1994): 574-578. 
3. Vingron, Martin, and Patrick Argos. "A fast and sensitive multiple sequence alignment algorithm." Bioinformatics 5.2 (1989): 115-121.