use rand::Rng;

const DECK: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

fn main() {
    println!("Welcome to Acey Ducey!");
    println!("How to play:");
    println!("The dealer deals out two cards face up. You have the option to bet");
    println!("or to pass based on whether you feel the next card will have a value");
    println!("between the two dealt cards. Aces are high. If you win, you earn double");
    println!("your bet.");
    println!("You start with $100.");
    println!("Enter 'p' to pass or 'q' to quit.");
    println!("-----\n");

    let mut wallet = 100; // This is the amount of money the player currently has.
    println!("Current wallet: ${}", wallet);
    play(&mut wallet);
}

fn play(wallet: &mut i32) {
    // Get a deck of cards, draw two cards and remove them from the deck.
    let mut cards: Vec<&str> = Vec::from(DECK);
    let dealer_index = rand::thread_rng().gen_range(0..cards.len());
    let dealer_card_1 = cards.remove(dealer_index);
    let dealer_index = rand::thread_rng().gen_range(0..cards.len());
    let dealer_card_2 = cards.remove(dealer_index);
    let player_index = rand::thread_rng().gen_range(0..cards.len());
    let player_card = cards.remove(player_index);

    // loop back over the reference deck to check for the values of the cards
    let mut dealer_card_1_val = 0;
    let mut dealer_card_2_val = 0;
    let mut player_card_val = 0;
    for (i, v) in DECK.iter().enumerate() {
        if *v == dealer_card_1 {
            dealer_card_1_val = i;
        }
        if *v == dealer_card_2 {
            dealer_card_2_val = i;
        }
        if *v == player_card {
            player_card_val = i;
        }
    }

    let dealer_cards = match dealer_card_1_val < dealer_card_2_val {
        true => format!("{}, {}", dealer_card_1, dealer_card_2),
        false => format!("{}, {}", dealer_card_2, dealer_card_1),
    };
    println!("Dealer cards: {}", dealer_cards);
    println!("Player card: {}", player_card);
    if (dealer_card_1_val < player_card_val && player_card_val < dealer_card_2_val)
        || (dealer_card_2_val < player_card_val && player_card_val < dealer_card_1_val)
    {
        println!("Player wins!");
    }
}
