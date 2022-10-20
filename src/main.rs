extern crate wordsearch;
use std::time::Instant;
use rand::seq::SliceRandom;
use rand::thread_rng;
use wordsearch::word_tree::WordTree;
use wordsearch::util::build_tree_from_file;
use wordsearch::word_tree::elements::WordElement;

const WORD_LENGTH: usize = 4;
const WORD_LIST: &str = "C:\\Users\\wille\\IdeaProjects\\wordsearch\\word_lists\\small.txt";

fn main() {

    let epoch = Instant::now();

    println!("[INIT] Initializing wordsquare");

    // make the board to hold our current solution
    let board: [[Option<char>; WORD_LENGTH]; WORD_LENGTH] = [[None; WORD_LENGTH]; WORD_LENGTH];

    // make full dictionary
    println!("[INIT][FULL] Building full dictionary using the wordlist at \"{WORD_LIST}\"");
    let start = Instant::now();
    let dictionary = build_tree_from_file(WORD_LIST);
    println!("[INIT][FULL] Built full dictionary in {}ms", start.elapsed().as_millis());

    // make a reduced dictionary
    println!("[INIT][REDUCE] Building reduced {WORD_LENGTH}-letter word dictionary");
    let start = Instant::now();

    // make the tree for the reduced dictionary
    let mut reduced_dict = WordTree::new();

    // get words of the desired length
    let reduced_wordlist = dictionary.find_words_of_length(WORD_LENGTH as u32);

    // add words of the correct length to the reduced dictionary
    for word in &reduced_wordlist {
        reduced_dict.insert(word);
    }

    println!("[INIT][REDUCE] Built reduced dictionary in {}ms", start.elapsed().as_millis());
    println!("[INIT][INFO] The reduced dictionary contains {} words.", reduced_wordlist.len());
    println!("[INIT] Initialization complete.");

    println!("[WDSQ] Starting wordsquare");
    println!("[WDSQ][SOLVE] Finding solutions...");
    let start = Instant::now();
    let solutions = get_solutions(board, &reduced_dict);
    println!("[WDSQ][SOLVE] {} solutions found in {}ms", solutions.len(), start.elapsed().as_millis());

    println!("[WDSQ][SHOW] Showing a random solution:");
    let rand_solution = solutions.choose(&mut thread_rng());
    match rand_solution {
        None => {println!("[WDSQ][SHOW][ERR] No solutions found!")}
        Some(solution) => {
            for row in solution {
                print!("             ");
                for letter in row {
                    match letter {
                        None => {panic!("Unexpected None in solution!")}
                        Some(c) => {print!("{}", c)}
                    }
                }
                println!();
            }
        }
    }

    // all done!
    println!("[WDSQ] All tasks completed in {}ms", epoch.elapsed().as_millis());
}

fn get_solutions(
    board: [[Option<char>; WORD_LENGTH]; WORD_LENGTH],
    dictionary: &WordTree
) -> Vec::<[[Option<char>; WORD_LENGTH]; WORD_LENGTH]> {
    let mut solutions = Vec::<[[Option<char>; WORD_LENGTH]; WORD_LENGTH]>::new();
    get_solution_recurse(board, &mut solutions, dictionary, (0, 0));
    return solutions
}

fn get_solution_recurse(
    mut board: [[Option<char>; WORD_LENGTH]; WORD_LENGTH],
    solution_list: &mut Vec::<[[Option<char>; WORD_LENGTH]; WORD_LENGTH]>,
    dictionary: &WordTree,
    coord: (u32, u32)
) {

    // build words for this coord
    let mut col_word = String::new();
    let mut row_word = String::new();

    for i in 0..WORD_LENGTH {
        match board[i as usize][coord.0 as usize] {
            None => {/* do nothing */}
            Some(letter) => {col_word.push(letter);}
        }

        match board[coord.1 as usize][i as usize] {
            None => {/* do nothing */}
            Some(letter) => {row_word.push(letter);}
        }
    }

    // find possible letters for this coordinate
    let col_poss = dictionary.suggest(&col_word);
    let row_poss = dictionary.suggest(&row_word);
    let mut common_poss = Vec::<char>::new();

    // find common possibilities
    if col_poss != None && row_poss != None {
        for col_elem in col_poss.unwrap() {
            match col_elem {
                WordElement::BeginWord => {panic!()}
                WordElement::Letter(col_letter) => {
                    for row_elem in row_poss.as_ref().unwrap() {
                        match row_elem {
                            WordElement::BeginWord => {panic!()}
                            WordElement::Letter(row_letter) => {
                                if col_letter == *row_letter {
                                    common_poss.push(col_letter.clone());
                                    break;
                                }
                            }
                            WordElement::EndWord => {/* do nothing */}
                        }
                    }
                }
                WordElement::EndWord => {/* do nothing */}
            }
        }
    }

    // get next coordinate to check
    let next_coord = get_next_coord(coord, WORD_LENGTH as u32);

    for letter in common_poss {
        board[coord.1 as usize][coord.0 as usize] = Some(letter);
        match next_coord {
            None => {
                // we found a solution! save it
                solution_list.push(board);
            }
            Some(c) => {
                // recurse
                get_solution_recurse(board, solution_list, dictionary, c);
            }
        }
    }

    board[coord.1 as usize][coord.0 as usize] = None;

}

fn get_next_coord(coord: (u32, u32), wordlength: u32) -> Option<(u32, u32)> {

    // make sure the input coord is in bounds
    if (coord.0 >= wordlength) && (coord.1 >= wordlength) {
        panic!("Input coord out of bounds!");
    }

    let low_side_y = coord.1 == 0;
    let high_side_x = coord.0 + 1 == wordlength;
    let high_side_y = coord.1 + 1 == wordlength;

    return if !low_side_y && !high_side_x {
        Some((coord.0 + 1, coord.1 - 1))
    } else if low_side_y && !high_side_x {
        Some((0, coord.0 + 1))
    } else if !high_side_y && high_side_x {
        Some((coord.1 + 1, wordlength - 1))
    } else {
        None
    }

}